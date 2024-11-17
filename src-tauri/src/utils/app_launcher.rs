pub struct App {
  pub name: String,
  pub icon: String,
  pub path: String,
  pub args: Vec<String>,
  pub terminal: bool,
  pub category: String,
}

pub struct AppLauncherManager {
  pub apps: Vec<App>,
}
