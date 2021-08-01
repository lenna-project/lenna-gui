#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use lenna_cli::plugins;
use lenna_core::Config;

#[tauri::command]
async fn get_config() -> Result<Config, String> {
  let config_file = std::fs::File::open("lenna.yml").unwrap();
  let config: Config = serde_yaml::from_reader(config_file).unwrap();
  Ok(config)
}

#[tauri::command]
async fn get_plugin_ids() -> Result<Vec<String>, String> {
  let mut plugins = plugins::Plugins::new();
  let plugins_path = match std::env::var("LENNA_PLUGINS") {
    Ok(val) => std::path::PathBuf::from(val),
    _ => std::path::PathBuf::from("plugins/"),
  };

  plugins.load_plugins(&plugins_path);

  let plugin_ids: Vec<String> = plugins.pool.ids();
  Ok(plugin_ids)
}

#[tauri::command]
fn my_custom_command() -> String {
  "Hello from Rust!".into()
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![my_custom_command, get_config, get_plugin_ids])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
