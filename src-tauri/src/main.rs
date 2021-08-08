#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use lenna_cli::{images_in_path, plugins};
use lenna_core::{Config, Pipeline, Pool, ProcessorConfig};
use scraper::{Html, Selector};
use serde_json::Value;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
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

#[derive(serde::Serialize, serde::Deserialize)]
struct Ui {
  template: String,
  script: String,
  style: String,
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

#[tauri::command]
async fn get_plugin_ui(state: tauri::State<'_, State>, id: String) -> Result<Ui, String> {
  let pool = state.pool.lock().unwrap();
  let plugin = pool.get(&id);
  match plugin {
    Some(plugin) => match plugin.config_ui() {
      Some(ui) => {
        let fragment = Html::parse_fragment(&ui);
        let template_selector = Selector::parse("template").unwrap();
        let template = fragment.select(&template_selector).next().unwrap();
        Ok(Ui {
          template: template.inner_html(),
          script: "".to_string(),
          style: "".to_string(),
        })
      }
      _ => Err("Plugin has no config ui.".to_string()),
    },
    _ => Err("No such plugin.".to_string()),
  }
}

#[tauri::command]
async fn process(
  state: tauri::State<'_, State>,
  source: String,
  target: String,
  extension: String,
) -> Result<(), String> {
  let config = state.config.lock().unwrap();
  let pool = state.pool.lock().unwrap();
  let pipeline = Pipeline::new(config.clone(), pool.clone());
  let path = PathBuf::from(source);
  for path in images_in_path(&path) {
    let img = lenna_core::io::read::read_from_file(path.to_str().unwrap().to_string());
    match img {
      Ok(img) => {
        let mut img = Box::new(img);
        pipeline.run(&mut img).unwrap();
        img.path = target.clone();
        lenna_core::io::write::write_to_file(&img, image::ImageOutputFormat::Jpeg(80)).unwrap();
      }, Err(err) => {
        println!("{:?}", err);
      }
    }
  }
  Ok(())
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
      get_plugin_ui,
      process
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
