#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use lenna_cli::plugins;
use lenna_core::{Config, Pool};

struct State(Pool);

#[derive(serde::Serialize)]
struct Plugin {
  name: String,
  description: String,
}

#[tauri::command]
async fn get_config() -> Result<Config, String> {
  let config_file = std::fs::File::open("lenna.yml").unwrap();
  let config: Config = serde_yaml::from_reader(config_file).unwrap();
  Ok(config)
}

#[tauri::command]
async fn get_plugin_ids(state: tauri::State<'_, State>) -> Result<Vec<String>, String> {
  let plugin_ids: Vec<String> = state.0.ids();
  Ok(plugin_ids)
}

#[tauri::command]
async fn get_plugin(state: tauri::State<'_, State>, id: String) -> Result<Plugin, String> {
  let plugin = state.0.get(&id);
  match plugin {
    Some(plugin) => Ok(Plugin {
      name: plugin.name(),
      description: plugin.description(),
    }),
    _ => Err("No such plugin".into()),
  }
}

#[tauri::command]
fn my_custom_command() -> String {
  "Hello from Rust!".into()
}

fn main() {
  let mut plugins = plugins::Plugins::new();
  let plugins_path = match std::env::var("LENNA_PLUGINS") {
    Ok(val) => std::path::PathBuf::from(val),
    _ => std::path::PathBuf::from("plugins/"),
  };

  plugins.load_plugins(&plugins_path);

  let pool: Pool = Pool::default();

  let state = State { 0: pool };

  tauri::Builder::default()
    .manage(state)
    .invoke_handler(tauri::generate_handler![
      my_custom_command,
      get_config,
      get_plugin_ids,
      get_plugin
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
