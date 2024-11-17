use tauri::{LogicalSize, Manager, Runtime};

use crate::utils::{widget::Widget, widget_handler::WidgetHandler};

#[tauri::command]
pub fn greet(name: &str) -> String {
  format!("Hello, {}! You've been greeted from Rust!", name)
}

// ========= Widget =========

#[tauri::command]
pub async fn get_widget_config<R: Runtime>(
  app: tauri::AppHandle<R>,
  window: tauri::Window<R>,
  widget_id: String,
) -> Result<Widget, String> {
  let widget_handler = app.state::<WidgetHandler>();

  let widget = widget_handler.get_widget(&widget_id).unwrap();

  Ok(widget)
}
