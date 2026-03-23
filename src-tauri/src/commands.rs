use std::path::PathBuf;
use std::sync::Mutex;

use anyhow::{anyhow, Result};
use serde::Deserialize;
use tauri::State;

use crate::db::Database;
use crate::models::{ScanRoot, SkillDetail, SkillRecord};
use crate::parser::parse_content;
use crate::scanner::{default_scan_roots, scan_roots};

pub struct AppState {
    pub db: Mutex<Database>,
}

impl AppState {
    pub fn new() -> Result<Self> {
        let mut base = dirs::data_local_dir().unwrap_or_else(std::env::temp_dir);
        base.push("skills-manager");
        std::fs::create_dir_all(&base)?;
        let db = Database::open(base.join("index.sqlite3"))?;
        Ok(Self { db: Mutex::new(db) })
    }
}

#[derive(Debug, Deserialize)]
pub struct ScanRequest {
    pub custom_paths: Option<Vec<String>>,
}

pub fn perform_scan(db: &Database, custom_paths: &[String]) -> Result<Vec<SkillRecord>> {
    let mut roots = default_scan_roots();
    for path in custom_paths {
        roots.push(ScanRoot {
            tool_source: "custom".into(),
            path: path.clone(),
            is_global: true,
        });
    }

    perform_scan_with_roots(db, &roots)
}

pub fn perform_scan_with_roots(db: &Database, roots: &[ScanRoot]) -> Result<Vec<SkillRecord>> {
    let skills = scan_roots(&roots)?;
    db.replace_scan(&skills)?;
    db.list_skills(None)
}

pub fn perform_get_skill(db: &Database, id: i64) -> Result<SkillDetail> {
    let record = db
        .get_skill(id)?
        .ok_or_else(|| anyhow!("Skill {id} not found"))?;
    let installations = db.installations_for(id)?;
    Ok(SkillDetail {
        record,
        installations,
    })
}

pub fn perform_save_skill(db: &Database, id: i64, content: &str) -> Result<SkillDetail> {
    let record = db
        .get_skill(id)?
        .ok_or_else(|| anyhow!("Skill {id} not found"))?;

    std::fs::write(&record.primary_path, content)?;
    let metadata = std::fs::metadata(&record.primary_path)?;
    let file_name = PathBuf::from(&record.primary_path)
        .file_name()
        .and_then(|value| value.to_str())
        .unwrap_or("SKILL.md")
        .to_string();
    let parsed = parse_content(content, &file_name);

    db.update_skill_content(
        id,
        content,
        if parsed.name.is_empty() {
            record.name.clone()
        } else {
            parsed.name
        },
        parsed.description,
        metadata
            .modified()
            .ok()
            .and_then(|value| value.duration_since(std::time::UNIX_EPOCH).ok())
            .map(|value| value.as_secs() as i64)
            .unwrap_or(record.file_modified_at),
        metadata.len() as i64,
    )?;

    perform_get_skill(db, id)
}

#[tauri::command]
pub fn scan_all(request: Option<ScanRequest>, state: State<'_, AppState>) -> Result<Vec<SkillRecord>, String> {
    let db = state.db.lock().map_err(|_| "database lock poisoned".to_string())?;
    perform_scan(&db, &request.and_then(|value| value.custom_paths).unwrap_or_default())
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn list_skills(query: Option<String>, state: State<'_, AppState>) -> Result<Vec<SkillRecord>, String> {
    let db = state.db.lock().map_err(|_| "database lock poisoned".to_string())?;
    db.list_skills(query.as_deref()).map_err(|error| error.to_string())
}

#[tauri::command]
pub fn get_skill(id: i64, state: State<'_, AppState>) -> Result<SkillDetail, String> {
    let db = state.db.lock().map_err(|_| "database lock poisoned".to_string())?;
    perform_get_skill(&db, id).map_err(|error| error.to_string())
}

#[tauri::command]
pub fn save_skill(id: i64, content: String, state: State<'_, AppState>) -> Result<SkillDetail, String> {
    let db = state.db.lock().map_err(|_| "database lock poisoned".to_string())?;
    perform_save_skill(&db, id, &content).map_err(|error| error.to_string())
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::tempdir;

    use crate::db::Database;
    use crate::models::{ScanRoot, ScannedSkill};

    #[test]
    fn scan_indexes_files_into_database() {
        let db = Database::open_in_memory().expect("db");
        let dir = tempdir().expect("tempdir");
        let skill_dir = dir.path().join("deploy-helper");
        fs::create_dir_all(&skill_dir).expect("create skill dir");
        fs::write(
            skill_dir.join("SKILL.md"),
            "---\nname: deploy-helper\ndescription: deploy safely\n---\n# ignore",
        )
        .expect("write skill");

        let records = super::perform_scan_with_roots(
            &db,
            &[ScanRoot {
                tool_source: "custom".into(),
                path: dir.path().to_string_lossy().to_string(),
                is_global: true,
            }],
        )
        .expect("scan");
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].name, "deploy-helper");
    }

    #[test]
    fn save_updates_disk_and_indexed_metadata() {
        let db = Database::open_in_memory().expect("db");
        let dir = tempdir().expect("tempdir");
        let file = dir.path().join("AGENTS.md");
        fs::write(&file, "# Original Name\n\nbody").expect("write skill");

        db.replace_scan(&[ScannedSkill {
            resolved_path: file.to_string_lossy().to_string(),
            primary_path: file.to_string_lossy().to_string(),
            name: "Original Name".into(),
            description: "".into(),
            content: "# Original Name\n\nbody".into(),
            tool_source: "codex".into(),
            format: "agents_md".into(),
            is_directory_skill: false,
            is_global: true,
            project_name: None,
            file_modified_at: 1,
            file_size: 12,
            installations: vec![],
        }])
        .expect("seed");

        let detail = super::perform_save_skill(&db, 1, "# Updated Name\n\nnew body").expect("save");
        assert_eq!(detail.record.name, "Updated Name");
        assert_eq!(fs::read_to_string(file).expect("read back"), "# Updated Name\n\nnew body");
    }
}
