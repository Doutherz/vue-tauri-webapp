// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod database;
use database::Task;
use rusqlite::Connection;

fn connect_db() -> Result<Connection, String> {
    database::init_db().map_err(|err| format!("Failed to connect to db: {}", err))
}

#[tauri::command]
fn create_task(title:&str, task:&str) -> Result<String, String> {
    let conn = connect_db()?;
    database::create_task(conn, title, task).map_err(|err| format!("Failed to create task: {}", err))?;
    Ok(format!("Created task"))
}

#[tauri::command]
fn fetch_todo() -> Result<Vec<Task>, String>{
    let conn = connect_db()?;
    Ok(database::fetch_todo(conn).map_err(|err| format!("Failed to fetch todo list: {}", err))?)
}

#[tauri::command]
fn remove_task(id:i32) -> Result<String, String> {
    let conn = connect_db()?;
    database::remove_task(conn, &id).map_err(|err| format!("Failed to remove task: {}", err))?;
    Ok(format!("Removed task {}", id))
}

#[tauri::command]
fn complete_task(id:i32) -> Result<String, String> {
    let conn = connect_db()?;
    database::complete_task(conn, &id).map_err(|err| format!("Failed to complete_task: {}", err))?;
    Ok(format!("Completed task: {}", id))
}

#[tauri::command]
fn uncomplete_task(id:i32) -> Result<String, String> {
    let conn = connect_db()?;
    database::uncomplete_task(conn, &id).map_err(|err| format!("Failed to uncomplete_task: {}", err))?;
    Ok(format!("Uncompleted task: {}", id))
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![create_task, fetch_todo, remove_task, complete_task, uncomplete_task])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
