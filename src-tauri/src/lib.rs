mod command;
mod utils;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use command::greet;
use utils::widget_handler::WidgetHandler;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .setup(|app| {
      tauri::async_runtime::block_on(async {
        let widget_handler = WidgetHandler::new(app);

        // for (i, widget) in widgets.iter().enumerate() {
        //   println!("{}: {}", i, serde_json::to_string_pretty(widget).unwrap());
        // }

        widget_handler.initialize_all_widgets(app);
      });

      Ok(())
    })
    .plugin(tauri_plugin_shell::init())
    .invoke_handler(tauri::generate_handler![greet])
    .invoke_handler(tauri::generate_handler![command::get_widget_config])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
