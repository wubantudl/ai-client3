#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use ai_client_lib::{commands, init_logging};

fn main() {
    init_logging();
    
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            commands::llm::chat,
            commands::llm::chat_stream,
            commands::llm::get_models,
            commands::llm::save_model_config,
            commands::llm::get_model_config,
            commands::mcp::connect_server,
            commands::mcp::disconnect_server,
            commands::mcp::list_servers,
            commands::mcp::call_tool,
            commands::skills::list_skills,
            commands::skills::import_skill,
            commands::skills::import_from_github,
            commands::skills::delete_skill,
            commands::skills::toggle_skill,
            commands::skills::get_skill_path,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
