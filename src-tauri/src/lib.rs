// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod db;
mod error;
mod notes;
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            db::setup(app)?;
            Ok(())
            })
        .invoke_handler(tauri::generate_handler![
            error::get_current_project,
            error::set_current_project,
            db::queries::list_projects,
            notes::list_notes,
            notes::read_note
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
