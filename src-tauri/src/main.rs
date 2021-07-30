#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use lenna_core::{Config};

#[tauri::command]
async fn get_config() -> Result<Config, String> {
  let config_file = std::fs::File::open("lenna.yml").unwrap();
  let config: Config = serde_yaml::from_reader(config_file).unwrap();
  Ok(config)
}

#[tauri::command]
fn my_custom_command() -> String {
  "Hello from Rust!".into()
}

fn main() {


  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![my_custom_command, get_config])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
