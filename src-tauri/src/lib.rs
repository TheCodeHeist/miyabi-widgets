mod command;
mod utils;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use command::greet;
use utils::system::get_basic_system_info;
// use utils::system::{get_cpu_usage, get_ram_usage, get_temperature};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .setup(|app| {
      // let widget_manager = utils::config::WidgetManager::new(app);

      // println!("{:?}", widget_manager.widgets);

      // initiate_media_control(app);

      // tauri::async_runtime::block_on(async move {
      //   loop {
      //     get_cpu_usage();
      //     get_ram_usage();
      //     get_temperature();

      //     std::thread::sleep(std::time::Duration::from_secs(
      //       sysinfo::MINIMUM_CPU_UPDATE_INTERVAL.as_secs() * 100,
      //     ));
      //   }
      // });

      get_basic_system_info();

      Ok(())
    })
    .plugin(tauri_plugin_shell::init())
    .invoke_handler(tauri::generate_handler![greet])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
