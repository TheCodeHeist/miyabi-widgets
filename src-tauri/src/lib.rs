mod command;
mod utils;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use command::greet;
use utils::system::initiate_system_info_fetcher;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .setup(|app| {
      initiate_system_info_fetcher(app).unwrap_or_else(|e| {
        eprintln!("Failed to initiate system info fetcher: {}", e);
      });

      Ok(())
    })
    .plugin(tauri_plugin_shell::init())
    .invoke_handler(tauri::generate_handler![greet])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
