export type ToolSource = "claude" | "cursor" | "codex" | "openclaw" | "custom";

export interface SkillRecord {
  id: number;
  resolved_path: string;
  primary_path: string;
  name: string;
  description: string;
  content: string;
  tool_source: ToolSource | string;
  format: string;
  is_directory_skill: boolean;
  is_global: boolean;
  project_name: string | null;
  file_modified_at: number;
  file_size: number;
}

export interface SkillInstallation {
  path: string;
  tool_source: ToolSource | string;
}

export interface SkillDetail {
  record: SkillRecord;
  installations: SkillInstallation[];
}
