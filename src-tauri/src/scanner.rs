use std::fs;
use std::path::{Path, PathBuf};
use std::time::UNIX_EPOCH;

use anyhow::Result;

use crate::models::{ScanRoot, ScannedSkill, SkillInstallation};
use crate::parser::parse_content;

pub fn default_scan_roots() -> Vec<ScanRoot> {
    let home = dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .to_string_lossy()
        .to_string();

    vec![
        ScanRoot {
            tool_source: "claude".into(),
            path: format!("{home}/.claude/skills"),
            is_global: true,
        },
        ScanRoot {
            tool_source: "claude".into(),
            path: format!("{home}/.agents/skills"),
            is_global: true,
        },
        ScanRoot {
            tool_source: "cursor".into(),
            path: format!("{home}/.cursor/skills"),
            is_global: true,
        },
        ScanRoot {
            tool_source: "cursor".into(),
            path: format!("{home}/.cursor/rules"),
            is_global: true,
        },
        ScanRoot {
            tool_source: "codex".into(),
            path: format!("{home}/.codex"),
            is_global: true,
        },
    ]
}

pub fn scan_roots(roots: &[ScanRoot]) -> Result<Vec<ScannedSkill>> {
    let mut scanned = Vec::new();

    for root in roots {
        let expanded = shellexpand::tilde(&root.path).to_string();
        let root_path = PathBuf::from(expanded);
        if !root_path.exists() {
            continue;
        }

        if root.tool_source == "codex" {
            collect_codex(&root_path, root, &mut scanned)?;
        } else {
            collect_standard(&root_path, root, &mut scanned)?;
        }
    }

    scanned.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    Ok(scanned)
}

fn collect_codex(root: &Path, scan_root: &ScanRoot, output: &mut Vec<ScannedSkill>) -> Result<()> {
    let root_agents = root.join("AGENTS.md");
    if root_agents.exists() {
        if let Some(skill) = build_skill(&root_agents, scan_root, false)? {
            output.push(skill);
        }
    }

    let nested_skills = root.join("skills");
    if nested_skills.exists() {
        collect_standard(&nested_skills, scan_root, output)?;
    }

    for entry in fs::read_dir(root)? {
        let entry = entry?;
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }

        let skill_md = path.join("SKILL.md");
        let agents_md = path.join("AGENTS.md");
        if skill_md.exists() {
            if let Some(skill) = build_skill(&skill_md, scan_root, true)? {
                output.push(skill);
            }
        } else if agents_md.exists() {
            if let Some(skill) = build_skill(&agents_md, scan_root, true)? {
                output.push(skill);
            }
        }
    }

    Ok(())
}

fn collect_standard(root: &Path, scan_root: &ScanRoot, output: &mut Vec<ScannedSkill>) -> Result<()> {
    for entry in fs::read_dir(root)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            let skill_md = path.join("SKILL.md");
            let agents_md = path.join("AGENTS.md");
            if skill_md.exists() {
                if let Some(skill) = build_skill(&skill_md, scan_root, true)? {
                    output.push(skill);
                }
            } else if agents_md.exists() {
                if let Some(skill) = build_skill(&agents_md, scan_root, true)? {
                    output.push(skill);
                }
            }
            continue;
        }

        let extension = path.extension().and_then(|value| value.to_str()).unwrap_or_default();
        let file_name = path.file_name().and_then(|value| value.to_str()).unwrap_or_default();
        if matches!(extension, "md" | "mdc") || file_name == "AGENTS.md" {
            if let Some(skill) = build_skill(&path, scan_root, false)? {
                output.push(skill);
            }
        }
    }

    Ok(())
}

fn build_skill(path: &Path, scan_root: &ScanRoot, is_directory_skill: bool) -> Result<Option<ScannedSkill>> {
    let content = match fs::read_to_string(path) {
        Ok(value) => value,
        Err(_) => return Ok(None),
    };
    let metadata = fs::metadata(path)?;
    let resolved_path = fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf());
    let file_name = path.file_name().and_then(|value| value.to_str()).unwrap_or("SKILL.md");
    let parsed = parse_content(&content, file_name);

    let name = if !parsed.name.is_empty() {
        parsed.name
    } else if is_directory_skill {
        path.parent()
            .and_then(Path::file_name)
            .and_then(|value| value.to_str())
            .unwrap_or("untitled")
            .to_string()
    } else {
        path.file_stem()
            .and_then(|value| value.to_str())
            .unwrap_or("untitled")
            .to_string()
    };

    let modified = metadata
        .modified()
        .ok()
        .and_then(|value| value.duration_since(UNIX_EPOCH).ok())
        .map(|value| value.as_secs() as i64)
        .unwrap_or_default();

    let primary_path = path.to_string_lossy().to_string();

    Ok(Some(ScannedSkill {
        resolved_path: resolved_path.to_string_lossy().to_string(),
        primary_path: primary_path.clone(),
        name,
        description: parsed.description,
        content,
        tool_source: scan_root.tool_source.clone(),
        format: parsed.format,
        is_directory_skill,
        is_global: scan_root.is_global,
        project_name: None,
        file_modified_at: modified,
        file_size: metadata.len() as i64,
        installations: vec![SkillInstallation {
            path: primary_path,
            tool_source: scan_root.tool_source.clone(),
        }],
    }))
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::tempdir;

    use crate::models::ScanRoot;

    #[test]
    fn scans_standard_skill_directories() {
        let dir = tempdir().expect("tempdir");
        let skill_dir = dir.path().join("skills").join("deploy-helper");
        fs::create_dir_all(&skill_dir).expect("create skill dir");
        fs::write(
            skill_dir.join("SKILL.md"),
            "---\nname: deploy-helper\ndescription: deploy safely\n---\n# ignore",
        )
        .expect("write skill");

        let results = super::scan_roots(&[ScanRoot {
            tool_source: "claude".into(),
            path: dir.path().join("skills").to_string_lossy().to_string(),
            is_global: true,
        }])
        .expect("scan roots");

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "deploy-helper");
        assert_eq!(results[0].description, "deploy safely");
        assert!(results[0].is_directory_skill);
    }

    #[test]
    fn scans_codex_agents_file() {
        let dir = tempdir().expect("tempdir");
        fs::write(dir.path().join("AGENTS.md"), "# Codex Rules\n\nBe precise.").expect("write agents");

        let results = super::scan_roots(&[ScanRoot {
            tool_source: "codex".into(),
            path: dir.path().to_string_lossy().to_string(),
            is_global: true,
        }])
        .expect("scan roots");

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Codex Rules");
        assert_eq!(results[0].format, "agents_md");
        assert!(!results[0].is_directory_skill);
    }
}
