mod commands;
mod db;
mod models;
mod parser;
mod scanner;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app_state = commands::AppState::new().expect("app state");
    tauri::Builder::default()
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            commands::scan_all,
            commands::list_skills,
            commands::get_skill,
            commands::save_skill
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
