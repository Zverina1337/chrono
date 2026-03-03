use std::sync::Mutex;

use rusqlite::Connection;
use tauri::State;

use crate::error::AppError;
use crate::models::priority::{CreatePriority, Priority, UpdatePriority};
use crate::repository::priority_repo;

#[tauri::command]
pub fn get_priorities_by_project(
  db: State<Mutex<Connection>>,
  project_uuid: String,
) -> Result<Vec<Priority>, AppError> {
  let conn = db.lock().unwrap();
  priority_repo::get_by_project(&conn, &project_uuid)
}

#[tauri::command]
pub fn create_priority(
  db: State<Mutex<Connection>>,
  data: CreatePriority,
) -> Result<Priority, AppError> {
  let conn = db.lock().unwrap();
  priority_repo::create(&conn, data)
}

#[tauri::command]
pub fn update_priority(
  db: State<Mutex<Connection>>,
  uuid: String,
  data: UpdatePriority,
) -> Result<Priority, AppError> {
  let conn = db.lock().unwrap();
  priority_repo::update(&conn, &uuid, data)
}

#[tauri::command]
pub fn delete_priority(db: State<Mutex<Connection>>, uuid: String) -> Result<(), AppError> {
  let conn = db.lock().unwrap();
  priority_repo::delete(&conn, &uuid)
}
