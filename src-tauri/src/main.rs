#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::fs::OpenOptions;
use std::io::prelude::*;
use std::env;

#[tauri::command]
fn add_task(content: String) {
  let path=env::var("FOCUS_TASKS_PATH").expect("The 'FOCUS_TASKS_PATH' environment variable is not set");
  let mut file = OpenOptions::new()
    .create(true)
    .append(true)
    .open(path)
    .expect("Error while opening the tasks file");
    writeln!(file, "{}","=> ".to_owned()+ &content).expect("Error while writing to the tasks file");
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![add_task])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
