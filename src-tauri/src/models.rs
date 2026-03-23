use serde::Serialize;

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct SkillRecord {
    pub id: i64,
    pub resolved_path: String,
    pub primary_path: String,
    pub name: String,
    pub description: String,
    pub content: String,
    pub tool_source: String,
    pub format: String,
    pub is_directory_skill: bool,
    pub is_global: bool,
    pub project_name: Option<String>,
    pub file_modified_at: i64,
    pub file_size: i64,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct SkillDetail {
    pub record: SkillRecord,
    pub installations: Vec<SkillInstallation>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct SkillInstallation {
    pub path: String,
    pub tool_source: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScannedSkill {
    pub resolved_path: String,
    pub primary_path: String,
    pub name: String,
    pub description: String,
    pub content: String,
    pub tool_source: String,
    pub format: String,
    pub is_directory_skill: bool,
    pub is_global: bool,
    pub project_name: Option<String>,
    pub file_modified_at: i64,
    pub file_size: i64,
    pub installations: Vec<SkillInstallation>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScanRoot {
    pub tool_source: String,
    pub path: String,
    pub is_global: bool,
}
