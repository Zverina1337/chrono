use std::sync::Mutex;

use rusqlite::Connection;
use tauri::State;

use crate::error::AppError;
use crate::models::status::{CreateStatus, Status, UpdateStatus};
use crate::repository::status_repo;

#[tauri::command]
pub fn get_statuses_by_project(
  db: State<Mutex<Connection>>,
  project_uuid: String,
) -> Result<Vec<Status>, AppError> {
  let conn = db.lock().unwrap();
  status_repo::get_by_project(&conn, &project_uuid)
}

#[tauri::command]
pub fn create_status(
  db: State<Mutex<Connection>>,
  data: CreateStatus,
) -> Result<Status, AppError> {
  let conn = db.lock().unwrap();
  status_repo::create(&conn, data)
}

#[tauri::command]
pub fn update_status(
  db: State<Mutex<Connection>>,
  uuid: String,
  data: UpdateStatus,
) -> Result<Status, AppError> {
  let conn = db.lock().unwrap();
  status_repo::update(&conn, &uuid, data)
}

#[tauri::command]
pub fn delete_status(db: State<Mutex<Connection>>, uuid: String) -> Result<(), AppError> {
  let conn = db.lock().unwrap();
  status_repo::delete(&conn, &uuid)
}
