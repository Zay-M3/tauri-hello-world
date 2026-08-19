mod db;
mod models;
mod repository;

use std::sync::Arc;

use tauri::{Manager, State};

use crate::db::Database;
use crate::models::Task;
use crate::repository::sqlite::SqliteTaskRepository;
use crate::repository::TaskRepository;

#[tauri::command]
fn add_task(
    title: String,
    repo: State<'_, Arc<SqliteTaskRepository>>,
) -> Result<Task, String> {
    repo.add(title).map_err(|e| e.to_string())
}

#[tauri::command]
fn list_tasks(repo: State<'_, Arc<SqliteTaskRepository>>) -> Result<Vec<Task>, String> {
    repo.list().map_err(|e| e.to_string())
}

#[tauri::command]
fn delete_task(id: i64, repo: State<'_, Arc<SqliteTaskRepository>>) -> Result<(), String> {
    repo.delete(id).map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let db = Database::init(app).expect("failed to init db");
            app.manage(Arc::new(db.repo()));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![add_task, list_tasks, delete_task])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
