// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod database;
use database::Task;

#[tauri::command]
fn create_task(title:&str, task:&str) -> Result<String, String> {
    let conn = database::init_db().map_err(|err| format!("Failed to connect to db: {}", err))?;
    database::create_task(conn, title, task).map_err(|err| format!("Failed to create task: {}", err))?;
    Ok(format!("Created task"))
}

#[tauri::command]
fn fetch_todo() -> Result<Vec<Task>, String>{
    let conn = database::init_db().map_err(|err| format!("Failed to connect to db: {}", err))?;
    Ok(database::fetch_todo(conn).map_err(|err| format!("Failed to fetch todo list: {}", err))?)
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![create_task, fetch_todo])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
