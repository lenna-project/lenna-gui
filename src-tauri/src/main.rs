#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use base64::encode;
use lenna_cli::{images_in_path, plugins, write_to_path};
use lenna_core::{Config, Pipeline, Pool, ProcessorConfig};
use scraper::{Html, Selector};
use serde_json::Value;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::api::path::resource_dir;
use tauri::{Menu, MenuItem, Submenu};
use tauri::{PackageInfo, Window};

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

#[derive(Clone, serde::Serialize)]
struct Payload {
  message: String,
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
async fn get_plugin_icon(state: tauri::State<'_, State>, id: String) -> Result<String, String> {
  let pool = state.pool.lock().unwrap();
  let plugin = pool.get(&id);
  match plugin {
    Some(plugin) => match plugin.icon() {
      Some(icon) => {
        let b64_img = encode(icon);
        let img = format!("data:image/jpeg;base64,{}", b64_img);
        Ok(img)
      }
      _ => Err("Plugin has no icon.".to_string()),
    },
    _ => Err("No such plugin.".to_string()),
  }
}

#[tauri::command]
async fn process(
  window: Window,
  state: tauri::State<'_, State>,
  source: String,
  target: String,
  extension: String,
) -> Result<(), String> {
  println!("Processing {} -> {} as {}", source, target, extension);
  let config = state.config.lock().unwrap();
  let pool = state.pool.lock().unwrap();
  let pipeline = Pipeline::new(config.clone(), pool.clone());
  let path = PathBuf::from(source);
  let mut count = 0;
  for path in images_in_path(&path) {
    let img = lenna_core::io::read::read_from_file(path.to_str().unwrap().to_string());
    match img {
      Ok(img) => {
        let mut img = Box::new(img);
        let name = img.name.clone();
        pipeline.run(&mut img).unwrap();

        write_to_path(img, target.clone(), extension.clone());
        window
          .emit(
            "info",
            Payload {
              message: format!("Saved image {}", name),
            },
          )
          .unwrap();
        count += 1;
      }
      Err(err) => {
        println!("{:?}", err);
      }
    }
  }
  window
    .emit(
      "success",
      Payload {
        message: format!("converted {} images", count),
      },
    )
    .unwrap();
  Ok(())
}

fn menu() -> Menu {
  Menu::new().add_submenu(Submenu::new(
    "Lenna",
    Menu::new()
      .add_native_item(MenuItem::Hide)
      .add_native_item(MenuItem::Quit),
  ))
}

fn main() {
  let mut plugins = plugins::Plugins::new();
  let package_info = PackageInfo {
    name: "lenna-gui".to_string(),
    version: "0.1.0".to_string(),
  };
  let plugins_path = match std::env::var("LENNA_PLUGINS") {
    Ok(val) => std::path::PathBuf::from(val),
    _ => match resource_dir(&package_info) {
      Some(path) => {
        let mut path = path.clone();
        path.push("plugins/".to_string());
        path
      }
      _ => std::path::PathBuf::from("plugins/"),
    },
  };

  let config_path = match resource_dir(&package_info) {
    Some(path) => {
      let mut path = path.clone();
      path.push("lenna.yml".to_string());
      path
    }
    _ => std::path::PathBuf::from("lenna.yml"),
  };
  plugins.load_plugins(&plugins_path);

  let pool: Pool = plugins.pool;

  let config: Config = match std::fs::File::open(config_path.to_str().unwrap()) {
    Ok(config_file) => serde_yaml::from_reader(config_file).unwrap(),
    Err(err) => {
      println!("{:?}", err);
      Config::default()
    }
  };

  let state = State {
    pool: Mutex::new(pool),
    config: Mutex::new(config),
  };

  tauri::Builder::default()
    .menu(menu())
    .manage(state)
    .invoke_handler(tauri::generate_handler![
      get_config,
      get_plugin_ids,
      get_plugin,
      get_plugin_config,
      set_plugin_config,
      get_plugin_ui,
      get_plugin_icon,
      process
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
