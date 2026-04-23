use crate::skills::{self, SkillInfo};

#[tauri::command]
pub fn list_skills() -> Result<Vec<SkillInfo>, String> {
    skills::list_skills().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn import_skill(source_path: String) -> Result<SkillInfo, String> {
    skills::import_local_skill(&source_path).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn import_from_github(repo_url: String) -> Result<SkillInfo, String> {
    skills::import_from_github_repo(&repo_url).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_skill(path: String) -> Result<(), String> {
    skills::delete_skill(&path).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn toggle_skill(path: String, enabled: bool) -> Result<(), String> {
    skills::toggle_skill(&path, enabled).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_skill_path() -> Result<String, String> {
    skills::get_skills_base_path()
        .map(|p| p.to_string_lossy().to_string())
        .map_err(|e| e.to_string())
}
