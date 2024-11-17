use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum DefaultOrientation {
  Horizontal,
  Vertical,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum WidgetTheme {
  Normal,
  Dynamic,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WidgetAppearance {
  pub theme: Option<WidgetTheme>,
  pub background_color: Option<String>,
  pub opacity: Option<f32>,
  pub border_size: Option<u32>,
  pub border_color: Option<String>,
  pub border_radius: Option<u32>,
  pub padding: Option<u32>,
  pub fontscale: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WidgetProperty {
  pub title: Option<String>,
  pub icon: Option<String>,
  pub hidden: Option<bool>,
  pub resizable: Option<bool>,
  pub draggable: Option<bool>,
  pub position: Option<(u32, u32)>,
  pub size: Option<(u32, u32)>,
  pub orientation: Option<DefaultOrientation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum WidgetType {
  DefaultDateTime,
  DefaultWeather,
  DefaultMediaPlayerControls,
  DefaultAppLauncher,
  Custom,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Widget {
  pub id: String,
  pub description: String,
  pub widget_type: WidgetType,
  pub property: WidgetProperty,
  pub appearance: WidgetAppearance,
  pub children: Option<Vec<Widget>>,
}
