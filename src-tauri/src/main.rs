#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use lenna_cli::plugins;
use lenna_core::{Config, Pool, ProcessorConfig};
use serde_json::Value;
use std::io::Write;
use std::fs::OpenOptions;
use std::sync::Mutex;

struct State {
  pool: Mutex<Pool>,
  config: Mutex<Config>,
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
  Ok(state.config.lock().unwrap().clone())
}

#[tauri::command]
async fn get_plugin_ids(state: tauri::State<'_, State>) -> Result<Vec<String>, String> {
  let pool = state.pool.lock().unwrap();
  let plugin_ids: Vec<String> = pool.ids();
  Ok(plugin_ids)
}

#[tauri::command]
async fn get_plugin(state: tauri::State<'_, State>, id: String) -> Result<Plugin, String> {
  let pool = state.pool.lock().unwrap();
  let plugin = pool.get(&id);
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
async fn get_plugin_config(
  state: tauri::State<'_, State>,
  id: String,
) -> Result<ProcessorConfig, String> {
  let lock = state.config.lock().unwrap();
  let config = lock.find(id);
  match config {
    Some(config) => Ok(config.clone()),
    _ => Err("No such plugin".into()),
  }
}

#[tauri::command]
async fn set_plugin_config(
  state: tauri::State<'_, State>,
  id: String,
  config: Value,
) -> Result<(), String> {
  let mut lock = state.config.lock().unwrap();
  lock.add(id, config.clone());

  let config: lenna_core::Config = lock.clone();

  match serde_yaml::to_string(&config) {
    Err(value) => Err(value.to_string()),
    Ok(str) => {
      let mut config_file = OpenOptions::new().write(true).open("lenna.yml").unwrap();
      config_file.write_all(str.as_bytes()).unwrap();
      Ok(())
    }
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

  let state = State {
    pool: Mutex::new(pool),
    config: Mutex::new(config),
  };

  tauri::Builder::default()
    .manage(state)
    .invoke_handler(tauri::generate_handler![
      get_config,
      get_plugin_ids,
      get_plugin,
      get_plugin_config,
      set_plugin_config,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
