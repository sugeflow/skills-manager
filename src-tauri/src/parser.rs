#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedContent {
    pub name: String,
    pub description: String,
    pub frontmatter: Vec<(String, String)>,
    pub format: String,
}

pub fn parse_content(content: &str, file_name: &str) -> ParsedContent {
    let format = if file_name.ends_with(".mdc") {
        "mdc"
    } else if file_name.eq_ignore_ascii_case("AGENTS.md") {
        "agents_md"
    } else {
        "skill_md"
    }
    .to_string();

    let lines: Vec<&str> = content.lines().collect();
    if lines.first().is_some_and(|line| line.trim() == "---") {
        let mut frontmatter = Vec::new();
        for line in lines.iter().skip(1) {
            let trimmed = line.trim();
            if trimmed == "---" {
                break;
            }
            if let Some((key, value)) = trimmed.split_once(':') {
                frontmatter.push((key.trim().to_string(), value.trim().to_string()));
            }
        }

        let name = frontmatter
            .iter()
            .find(|(key, _)| key == "name")
            .map(|(_, value)| value.clone())
            .unwrap_or_default();
        let description = frontmatter
            .iter()
            .find(|(key, _)| key == "description")
            .map(|(_, value)| value.clone())
            .unwrap_or_default();

        if !name.is_empty() || !description.is_empty() {
            return ParsedContent {
                name,
                description,
                frontmatter,
                format,
            };
        }
    }

    let heading = lines
        .iter()
        .find_map(|line| line.strip_prefix("# "))
        .unwrap_or_default()
        .trim()
        .to_string();

    ParsedContent {
        name: heading,
        description: String::new(),
        frontmatter: Vec::new(),
        format,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn parses_frontmatter_name_and_description() {
        let parsed = super::parse_content(
            "---\nname: deploy-helper\ndescription: Ship code safely\n---\n# ignored\nUse this skill.",
            "SKILL.md",
        );

        assert_eq!(parsed.name, "deploy-helper");
        assert_eq!(parsed.description, "Ship code safely");
        assert_eq!(parsed.format, "skill_md");
    }

    #[test]
    fn falls_back_to_first_heading_without_frontmatter() {
        let parsed = super::parse_content("# Debug Helper\n\nCheck logs first.", "AGENTS.md");

        assert_eq!(parsed.name, "Debug Helper");
        assert_eq!(parsed.description, "");
        assert_eq!(parsed.format, "agents_md");
    }
}
