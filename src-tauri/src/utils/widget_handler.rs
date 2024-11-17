use tauri::{webview::WebviewWindowBuilder, App, LogicalPosition, Manager, WindowBuilder};

use super::widget::{DefaultOrientation, Widget, WidgetType};

#[derive(Clone)]
pub struct WidgetHandler {
  pub widgets: Vec<Widget>,
}

impl WidgetHandler {
  pub fn new(app: &mut App) -> Self {
    let config_path_dir = app
      .path()
      .app_config_dir()
      .unwrap_or_else(|_| panic!("failed to get app config dir"));

    if !config_path_dir.exists() {
      std::fs::create_dir_all(&config_path_dir)
        .unwrap_or_else(|_| panic!("failed to create config dir"));
    }

    let config_path = config_path_dir.join("./widgets/");
    if !config_path.exists() {
      std::fs::create_dir_all(&config_path)
        .unwrap_or_else(|_| panic!("failed to create widgets dir"));
    }

    let num_widgets = config_path.read_dir().unwrap().count();

    if num_widgets == 0 {
      Self {
        widgets: Vec::new(),
      }
    } else {
      let mut widgets = Vec::new();

      // read all the TOML files in the widgets directory
      for entry in config_path.read_dir().unwrap() {
        let entry = entry.unwrap_or_else(|_| panic!("failed to read entry in widgets dir"));
        let path = entry.path();
        let file_name = path.file_name().unwrap().to_str().unwrap().to_string();
        let file_stem = path.file_stem().unwrap().to_str().unwrap().to_string();

        if path.is_file() && file_name.ends_with(".json") {
          let config: Widget = serde_json::from_str(&std::fs::read_to_string(&path).unwrap())
            .unwrap_or_else(|e| {
              panic!(
                "failed to parse config file: {} | Error: {:?}",
                file_stem, e
              )
            });

          widgets.push(config);
        }
      }

      Self { widgets }
    }
  }

  pub fn initialize_all_widgets(self, app: &mut App) {
    let mut widget_handler = Self::new(app);
    let app_handle = app.handle();

    for widget in widget_handler.widgets.iter_mut() {
      match widget.widget_type {
        WidgetType::DefaultMediaPlayerControls => {
          let mut window = WebviewWindowBuilder::new(
            app_handle,
            widget.id.clone(),
            tauri::WebviewUrl::App(format!("/media?id={}", &widget.id).into()),
          )
          .decorations(false)
          .transparent(true)
          .skip_taskbar(true)
          .always_on_bottom(true)
          .resizable(widget.property.resizable.unwrap_or(false));

          if widget.property.position.is_some() {
            let position = widget.property.position.unwrap();
            window = window.position(position.0 as f64, position.1 as f64);
          } else {
            window = window.center();
          }

          if widget.property.orientation.is_some() {
            let orientation = widget.property.orientation.clone().unwrap();
            match orientation {
              DefaultOrientation::Horizontal => {
                window = window.inner_size(640.0, 160.0);
              }
              DefaultOrientation::Vertical => {
                window = window.inner_size(320.0, 480.0);
              }
            }
          } else {
            window = window.inner_size(320.0, 480.0);
          }

          window.build().unwrap();

          crate::utils::media::initiate_media_control(app).unwrap_or_else(|e| {
            eprintln!("Failed to initiate media control: {}", e);
          });
        }
        _ => todo!(),
      }
    }

    app.manage(widget_handler);
  }

  pub fn get_widget(&self, widget_id: &str) -> Option<Widget> {
    self
      .widgets
      .iter()
      .find(|widget| widget.id == widget_id)
      .cloned()
  }
}
