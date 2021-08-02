#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use lenna_cli::plugins;
use lenna_core::{Config, Pool, ProcessorConfig};

struct State {
  pool: Pool,
  config: Config
}

#[derive(serde::Serialize)]
struct Plugin {
  id: String,
  name: String,
  description: String,
  config: serde_json::Value,
}

#[tauri::command]
async fn get_config(state: tauri::State<'_, State>) -> Result<Config, String> {
  Ok(state.config.clone())
}

#[tauri::command]
async fn get_plugin_ids(state: tauri::State<'_, State>) -> Result<Vec<String>, String> {
  let plugin_ids: Vec<String> = state.pool.ids();
  Ok(plugin_ids)
}

#[tauri::command]
async fn get_plugin(state: tauri::State<'_, State>, id: String) -> Result<Plugin, String> {
  let plugin = state.pool.get(&id);
  match plugin {
    Some(plugin) => Ok(Plugin {
      id: plugin.id(),
      name: plugin.name(),
      description: plugin.description(),
      config: plugin.default_config(),
    }),
    _ => Err("No such plugin".into()),
  }
}

#[tauri::command]
async fn get_plugin_config(state: tauri::State<'_, State>, id: String) -> Result<ProcessorConfig, String> {
  let config = state.config.find(id);
  match config {
    Some(config) => Ok(config.clone()),
    _ => Err("No such plugin".into()),
  }
}

fn main() {
  let mut plugins = plugins::Plugins::new();
  let plugins_path = match std::env::var("LENNA_PLUGINS") {
    Ok(val) => std::path::PathBuf::from(val),
    _ => std::path::PathBuf::from("plugins/"),
  };

  plugins.load_plugins(&plugins_path);

  let pool: Pool = Pool::default();

  let config_file = std::fs::File::open("lenna.yml").unwrap();
  let config: Config = serde_yaml::from_reader(config_file).unwrap();

  let state = State { pool, config };

  tauri::Builder::default()
    .manage(state)
    .invoke_handler(tauri::generate_handler![
      get_config,
      get_plugin_ids,
      get_plugin,
      get_plugin_config
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
