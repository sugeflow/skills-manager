use std::path::PathBuf;

use anyhow::Result;
use rusqlite::{params, Connection};

use crate::models::{ScannedSkill, SkillInstallation, SkillRecord};

pub struct Database {
    path: PathBuf,
}

impl Database {
    pub fn open(path: PathBuf) -> Result<Self> {
        let db = Self { path };
        db.init()?;
        Ok(db)
    }

    #[cfg(test)]
    pub fn open_in_memory() -> Result<Self> {
        let unique = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();
        let path = std::env::temp_dir().join(format!("skills-manager-{unique}.sqlite3"));
        Self::open(path)
    }

    fn connect(&self) -> Result<Connection> {
        Ok(Connection::open(&self.path)?)
    }

    fn init(&self) -> Result<()> {
        let conn = self.connect()?;
        conn.execute_batch(
            "
            PRAGMA foreign_keys = ON;

            CREATE TABLE IF NOT EXISTS skills (
              id INTEGER PRIMARY KEY AUTOINCREMENT,
              resolved_path TEXT NOT NULL UNIQUE,
              primary_path TEXT NOT NULL,
              name TEXT NOT NULL,
              description TEXT NOT NULL,
              content TEXT NOT NULL,
              tool_source TEXT NOT NULL,
              format TEXT NOT NULL,
              is_directory_skill INTEGER NOT NULL,
              is_global INTEGER NOT NULL,
              project_name TEXT,
              file_modified_at INTEGER NOT NULL,
              file_size INTEGER NOT NULL,
              created_at INTEGER NOT NULL DEFAULT (unixepoch()),
              updated_at INTEGER NOT NULL DEFAULT (unixepoch())
            );

            CREATE TABLE IF NOT EXISTS skill_installations (
              id INTEGER PRIMARY KEY AUTOINCREMENT,
              skill_id INTEGER NOT NULL,
              path TEXT NOT NULL,
              tool_source TEXT NOT NULL,
              UNIQUE(skill_id, path),
              FOREIGN KEY (skill_id) REFERENCES skills(id) ON DELETE CASCADE
            );
            ",
        )?;
        Ok(())
    }

    pub fn replace_scan(&self, skills: &[ScannedSkill]) -> Result<()> {
        let mut conn = self.connect()?;
        let tx = conn.transaction()?;
        let mut seen: Vec<String> = Vec::with_capacity(skills.len());

        for skill in skills {
            seen.push(skill.resolved_path.clone());
            tx.execute(
                "
                INSERT INTO skills (
                  resolved_path, primary_path, name, description, content, tool_source, format,
                  is_directory_skill, is_global, project_name, file_modified_at, file_size, updated_at
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, unixepoch())
                ON CONFLICT(resolved_path) DO UPDATE SET
                  primary_path = excluded.primary_path,
                  name = excluded.name,
                  description = excluded.description,
                  content = excluded.content,
                  tool_source = excluded.tool_source,
                  format = excluded.format,
                  is_directory_skill = excluded.is_directory_skill,
                  is_global = excluded.is_global,
                  project_name = excluded.project_name,
                  file_modified_at = excluded.file_modified_at,
                  file_size = excluded.file_size,
                  updated_at = unixepoch()
                ",
                params![
                    skill.resolved_path,
                    skill.primary_path,
                    skill.name,
                    skill.description,
                    skill.content,
                    skill.tool_source,
                    skill.format,
                    skill.is_directory_skill as i64,
                    skill.is_global as i64,
                    skill.project_name,
                    skill.file_modified_at,
                    skill.file_size,
                ],
            )?;

            let skill_id: i64 = tx.query_row(
                "SELECT id FROM skills WHERE resolved_path = ?1",
                params![skill.resolved_path],
                |row| row.get(0),
            )?;

            tx.execute(
                "DELETE FROM skill_installations WHERE skill_id = ?1",
                params![skill_id],
            )?;

            for installation in &skill.installations {
                tx.execute(
                    "INSERT INTO skill_installations (skill_id, path, tool_source) VALUES (?1, ?2, ?3)",
                    params![skill_id, installation.path, installation.tool_source],
                )?;
            }
        }

        if seen.is_empty() {
            tx.execute("DELETE FROM skills", [])?;
        } else {
            let placeholders = std::iter::repeat_n("?", seen.len()).collect::<Vec<_>>().join(", ");
            let sql = format!("DELETE FROM skills WHERE resolved_path NOT IN ({placeholders})");
            let params = rusqlite::params_from_iter(seen.iter());
            tx.execute(&sql, params)?;
        }

        tx.commit()?;
        Ok(())
    }

    pub fn list_skills(&self, query: Option<&str>) -> Result<Vec<SkillRecord>> {
        let conn = self.connect()?;
        let mut items = Vec::new();

        if let Some(query) = query {
            let like = format!("%{query}%");
            let mut stmt = conn.prepare(
                "
                SELECT id, resolved_path, primary_path, name, description, content, tool_source, format,
                       is_directory_skill, is_global, project_name, file_modified_at, file_size
                FROM skills
                WHERE name LIKE ?1 OR description LIKE ?1 OR content LIKE ?1
                ORDER BY name COLLATE NOCASE
                ",
            )?;
            let rows = stmt.query_map(params![like], map_skill_row)?;
            for row in rows {
                items.push(row?);
            }
        } else {
            let mut stmt = conn.prepare(
                "
                SELECT id, resolved_path, primary_path, name, description, content, tool_source, format,
                       is_directory_skill, is_global, project_name, file_modified_at, file_size
                FROM skills
                ORDER BY name COLLATE NOCASE
                ",
            )?;
            let rows = stmt.query_map([], map_skill_row)?;
            for row in rows {
                items.push(row?);
            }
        }

        Ok(items)
    }

    pub fn installations_for(&self, skill_id: i64) -> Result<Vec<SkillInstallation>> {
        let conn = self.connect()?;
        let mut stmt = conn.prepare(
            "SELECT path, tool_source FROM skill_installations WHERE skill_id = ?1 ORDER BY path",
        )?;
        let rows = stmt.query_map(params![skill_id], |row| {
            Ok(SkillInstallation {
                path: row.get(0)?,
                tool_source: row.get(1)?,
            })
        })?;

        let mut items = Vec::new();
        for row in rows {
            items.push(row?);
        }
        Ok(items)
    }

    pub fn get_skill(&self, id: i64) -> Result<Option<SkillRecord>> {
        let conn = self.connect()?;
        let mut stmt = conn.prepare(
            "
            SELECT id, resolved_path, primary_path, name, description, content, tool_source, format,
                   is_directory_skill, is_global, project_name, file_modified_at, file_size
            FROM skills
            WHERE id = ?1
            ",
        )?;

        match stmt.query_row(params![id], map_skill_row) {
            Ok(record) => Ok(Some(record)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(error) => Err(error.into()),
        }
    }

    pub fn update_skill_content(
        &self,
        id: i64,
        content: &str,
        name: String,
        description: String,
        file_modified_at: i64,
        file_size: i64,
    ) -> Result<()> {
        let conn = self.connect()?;
        conn.execute(
            "
            UPDATE skills
            SET content = ?2,
                name = ?3,
                description = ?4,
                file_modified_at = ?5,
                file_size = ?6,
                updated_at = unixepoch()
            WHERE id = ?1
            ",
            params![id, content, name, description, file_modified_at, file_size],
        )?;
        Ok(())
    }
}

fn map_skill_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<SkillRecord> {
    Ok(SkillRecord {
        id: row.get(0)?,
        resolved_path: row.get(1)?,
        primary_path: row.get(2)?,
        name: row.get(3)?,
        description: row.get(4)?,
        content: row.get(5)?,
        tool_source: row.get(6)?,
        format: row.get(7)?,
        is_directory_skill: row.get::<_, i64>(8)? != 0,
        is_global: row.get::<_, i64>(9)? != 0,
        project_name: row.get(10)?,
        file_modified_at: row.get(11)?,
        file_size: row.get(12)?,
    })
}

#[cfg(test)]
mod tests {
    use crate::models::{ScannedSkill, SkillInstallation};

    #[test]
    fn initializes_schema_and_upserts_skills() {
        let db = super::Database::open_in_memory().expect("in-memory db");

        let first = ScannedSkill {
            resolved_path: "/tmp/skills/example/SKILL.md".into(),
            primary_path: "/tmp/skills/example/SKILL.md".into(),
            name: "example".into(),
            description: "first description".into(),
            content: "# example".into(),
            tool_source: "claude".into(),
            format: "skill_md".into(),
            is_directory_skill: true,
            is_global: true,
            project_name: None,
            file_modified_at: 100,
            file_size: 12,
            installations: vec![SkillInstallation {
                path: "/tmp/skills/example/SKILL.md".into(),
                tool_source: "claude".into(),
            }],
        };

        db.replace_scan(&[first.clone()]).expect("insert scan");

        let records = db.list_skills(None).expect("list skills");
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].name, "example");
        assert_eq!(records[0].description, "first description");

        let updated = ScannedSkill {
            description: "updated description".into(),
            content: "# updated".into(),
            file_modified_at: 200,
            file_size: 24,
            installations: vec![
                SkillInstallation {
                    path: "/tmp/skills/example/SKILL.md".into(),
                    tool_source: "claude".into(),
                },
                SkillInstallation {
                    path: "/tmp/links/example/SKILL.md".into(),
                    tool_source: "cursor".into(),
                },
            ],
            ..first
        };

        db.replace_scan(&[updated]).expect("update scan");

        let records = db.list_skills(Some("updated")).expect("list filtered");
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].description, "updated description");

        let installations = db.installations_for(records[0].id).expect("installations");
        assert_eq!(installations.len(), 2);
    }

    #[test]
    fn removes_stale_rows_after_scan() {
        let db = super::Database::open_in_memory().expect("in-memory db");

        let first = ScannedSkill {
            resolved_path: "/tmp/skills/one/SKILL.md".into(),
            primary_path: "/tmp/skills/one/SKILL.md".into(),
            name: "one".into(),
            description: "".into(),
            content: "# one".into(),
            tool_source: "claude".into(),
            format: "skill_md".into(),
            is_directory_skill: true,
            is_global: true,
            project_name: None,
            file_modified_at: 100,
            file_size: 5,
            installations: vec![SkillInstallation {
                path: "/tmp/skills/one/SKILL.md".into(),
                tool_source: "claude".into(),
            }],
        };

        let second = ScannedSkill {
            resolved_path: "/tmp/skills/two/SKILL.md".into(),
            primary_path: "/tmp/skills/two/SKILL.md".into(),
            name: "two".into(),
            description: "".into(),
            content: "# two".into(),
            tool_source: "codex".into(),
            format: "agents_md".into(),
            is_directory_skill: false,
            is_global: true,
            project_name: None,
            file_modified_at: 101,
            file_size: 6,
            installations: vec![SkillInstallation {
                path: "/tmp/skills/two/SKILL.md".into(),
                tool_source: "codex".into(),
            }],
        };

        db.replace_scan(&[first.clone(), second]).expect("insert scan");
        db.replace_scan(&[first]).expect("remove stale");

        let records = db.list_skills(None).expect("list skills");
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].name, "one");
    }
}
