use std::sync::Mutex;

use rusqlite::Connection;
use tauri::State;

use crate::error::AppError;
use crate::models::status::{CreateStatus, Status, UpdateStatus};
use crate::repository::status_repo;

fn lock_db(db: &Mutex<Connection>) -> Result<std::sync::MutexGuard<'_, Connection>, AppError> {
  db.lock().map_err(|e| {
    log::error!("Mutex poisoned: {e}");
    AppError::LockError
  })
}

#[tauri::command]
pub fn get_statuses_by_project(
  db: State<Mutex<Connection>>,
  project_uuid: String,
) -> Result<Vec<Status>, AppError> {
  let conn = lock_db(&db)?;
  status_repo::get_by_project(&conn, &project_uuid)
}

#[tauri::command]
pub fn create_status(
  db: State<Mutex<Connection>>,
  data: CreateStatus,
) -> Result<Status, AppError> {
  if data.name.trim().is_empty() {
    return Err(AppError::Validation("Имя статуса не может быть пустым".into()));
  }
  if data.name.len() > 255 {
    return Err(AppError::Validation("Имя статуса слишком длинное (макс. 255)".into()));
  }
  let conn = lock_db(&db)?;
  status_repo::create(&conn, data)
}

#[tauri::command]
pub fn update_status(
  db: State<Mutex<Connection>>,
  uuid: String,
  data: UpdateStatus,
) -> Result<Status, AppError> {
  if let Some(name) = &data.name {
    if name.trim().is_empty() {
      return Err(AppError::Validation("Имя статуса не может быть пустым".into()));
    }
    if name.len() > 255 {
      return Err(AppError::Validation("Имя статуса слишком длинное (макс. 255)".into()));
    }
  }
  let conn = lock_db(&db)?;
  status_repo::update(&conn, &uuid, data)
}

#[tauri::command]
pub fn delete_status(db: State<Mutex<Connection>>, uuid: String) -> Result<(), AppError> {
  let conn = lock_db(&db)?;
  status_repo::delete(&conn, &uuid)
}
