use std::sync::Mutex;

use rusqlite::Connection;
use tauri::State;

use crate::error::AppError;
use crate::models::priority::{CreatePriority, Priority, UpdatePriority};
use crate::repository::priority_repo;

fn lock_db(db: &Mutex<Connection>) -> Result<std::sync::MutexGuard<'_, Connection>, AppError> {
  db.lock().map_err(|e| {
    log::error!("Mutex poisoned: {e}");
    AppError::LockError
  })
}

#[tauri::command]
pub fn get_priorities_by_project(
  db: State<Mutex<Connection>>,
  project_uuid: String,
) -> Result<Vec<Priority>, AppError> {
  let conn = lock_db(&db)?;
  priority_repo::get_by_project(&conn, &project_uuid)
}

#[tauri::command]
pub fn create_priority(
  db: State<Mutex<Connection>>,
  data: CreatePriority,
) -> Result<Priority, AppError> {
  if data.name.trim().is_empty() {
    return Err(AppError::Validation("Имя приоритета не может быть пустым".into()));
  }
  if data.name.len() > 255 {
    return Err(AppError::Validation("Имя приоритета слишком длинное (макс. 255)".into()));
  }
  let conn = lock_db(&db)?;
  priority_repo::create(&conn, data)
}

#[tauri::command]
pub fn update_priority(
  db: State<Mutex<Connection>>,
  uuid: String,
  data: UpdatePriority,
) -> Result<Priority, AppError> {
  if let Some(name) = &data.name {
    if name.trim().is_empty() {
      return Err(AppError::Validation("Имя приоритета не может быть пустым".into()));
    }
    if name.len() > 255 {
      return Err(AppError::Validation("Имя приоритета слишком длинное (макс. 255)".into()));
    }
  }
  let conn = lock_db(&db)?;
  priority_repo::update(&conn, &uuid, data)
}

#[tauri::command]
pub fn delete_priority(db: State<Mutex<Connection>>, uuid: String) -> Result<(), AppError> {
  let conn = lock_db(&db)?;
  priority_repo::delete(&conn, &uuid)
}
