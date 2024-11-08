mod command;
mod utils;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use command::greet;
use utils::media::initiate_media_control;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .setup(|app| {
      // let widget_manager = utils::config::WidgetManager::new(app);

      // println!("{:?}", widget_manager.widgets);

      initiate_media_control(app);

      Ok(())
    })
    .plugin(tauri_plugin_shell::init())
    .invoke_handler(tauri::generate_handler![greet])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
