import { invoke } from "@tauri-apps/api/core";

import type { SkillDetail, SkillRecord } from "../types";

export async function scanAll(customPaths: string[] = []) {
  return invoke<SkillRecord[]>("scan_all", {
    request: customPaths.length > 0 ? { customPaths } : null,
  });
}

export async function listSkills(query: string) {
  return invoke<SkillRecord[]>("list_skills", {
    query: query.trim() ? query : null,
  });
}

export async function getSkill(id: number) {
  return invoke<SkillDetail>("get_skill", { id });
}

export async function saveSkill(id: number, content: string) {
  return invoke<SkillDetail>("save_skill", { id, content });
}
