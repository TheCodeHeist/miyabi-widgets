use serde::Deserialize;
use std::collections::HashMap;
use tauri::{App, Manager};

#[derive(Deserialize, Debug)]
pub struct Config {
  pub title: String,
  pub widgettype: String,
  pub hidden: bool,
  pub appearance: Appearance,
}

#[derive(Deserialize, Debug)]
pub struct Appearance {
  pub bgcolor: String,
  pub border: u32,
  pub borderrad: u32,
  pub alignment: String,
  pub fontscale: f32,
}
pub struct WidgetManager {
  pub widgets: HashMap<String, Config>,
}

impl WidgetManager {
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
        widgets: HashMap::new(),
      }
    } else {
      let mut widgets = Vec::new();

      // read all the TOML files in the widgets directory
      for entry in config_path.read_dir().unwrap() {
        let entry = entry.unwrap_or_else(|_| panic!("failed to read entry in widgets dir"));
        let path = entry.path();
        let file_name = path.file_name().unwrap().to_str().unwrap().to_string();
        let file_stem = path.file_stem().unwrap().to_str().unwrap().to_string();

        if path.is_file() && file_name.ends_with(".toml") {
          let config: Config = toml::from_str(&std::fs::read_to_string(&path).unwrap())
            .unwrap_or_else(|e| {
              panic!(
                "failed to parse config file: {} | Error: {:?}",
                file_stem, e
              )
            });

          widgets.push((file_stem, config));
        }
      }

      Self {
        widgets: widgets.into_iter().collect(),
      }
    }
  }
}
