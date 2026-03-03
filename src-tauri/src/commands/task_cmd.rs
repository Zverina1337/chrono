use std::sync::Mutex;

use rusqlite::Connection;
use tauri::State;

use crate::error::AppError;
use crate::models::task::{CreateTask, Task, UpdateTask};
use crate::repository::task_repo;

#[tauri::command]
pub fn get_tasks_by_project(
  db: State<Mutex<Connection>>,
  project_uuid: String,
) -> Result<Vec<Task>, AppError> {
  let conn = db.lock().unwrap();
  task_repo::get_by_project(&conn, &project_uuid)
}

#[tauri::command]
pub fn get_task(db: State<Mutex<Connection>>, uuid: String) -> Result<Task, AppError> {
  let conn = db.lock().unwrap();
  task_repo::get_by_uuid(&conn, &uuid)
}

#[tauri::command]
pub fn create_task(db: State<Mutex<Connection>>, data: CreateTask) -> Result<Task, AppError> {
  let conn = db.lock().unwrap();
  task_repo::create(&conn, data)
}

#[tauri::command]
pub fn update_task(
  db: State<Mutex<Connection>>,
  uuid: String,
  data: UpdateTask,
) -> Result<Task, AppError> {
  let conn = db.lock().unwrap();
  task_repo::update(&conn, &uuid, data)
}

#[tauri::command]
pub fn delete_task(db: State<Mutex<Connection>>, uuid: String) -> Result<(), AppError> {
  let conn = db.lock().unwrap();
  task_repo::delete(&conn, &uuid)
}
