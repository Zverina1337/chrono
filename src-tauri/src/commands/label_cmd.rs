use std::sync::Mutex;

use rusqlite::Connection;
use tauri::State;

use crate::error::AppError;
use crate::models::label::{CreateLabel, Label};
use crate::repository::label_repo;

fn lock_db(db: &Mutex<Connection>) -> Result<std::sync::MutexGuard<'_, Connection>, AppError> {
  db.lock().map_err(|e| {
    log::error!("Mutex poisoned: {e}");
    AppError::LockError
  })
}

#[tauri::command]
pub fn get_all_labels(db: State<Mutex<Connection>>) -> Result<Vec<Label>, AppError> {
  let conn = lock_db(&db)?;
  label_repo::get_all(&conn)
}

#[tauri::command]
pub fn create_label(db: State<Mutex<Connection>>, data: CreateLabel) -> Result<Label, AppError> {
  if data.name.trim().is_empty() {
    return Err(AppError::Validation("Имя лейбла не может быть пустым".into()));
  }
  if data.name.len() > 255 {
    return Err(AppError::Validation("Имя лейбла слишком длинное (макс. 255)".into()));
  }
  let conn = lock_db(&db)?;
  label_repo::create(&conn, data)
}

#[tauri::command]
pub fn delete_label(db: State<Mutex<Connection>>, uuid: String) -> Result<(), AppError> {
  let conn = lock_db(&db)?;
  label_repo::delete(&conn, &uuid)
}

#[tauri::command]
pub fn add_label_to_task(
  db: State<Mutex<Connection>>,
  task_uuid: String,
  label_uuid: String,
) -> Result<(), AppError> {
  let conn = lock_db(&db)?;
  label_repo::add_to_task(&conn, &task_uuid, &label_uuid)
}

#[tauri::command]
pub fn remove_label_from_task(
  db: State<Mutex<Connection>>,
  task_uuid: String,
  label_uuid: String,
) -> Result<(), AppError> {
  let conn = lock_db(&db)?;
  label_repo::remove_from_task(&conn, &task_uuid, &label_uuid)
}
