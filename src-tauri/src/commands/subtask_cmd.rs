use std::sync::Mutex;

use rusqlite::Connection;
use tauri::State;

use crate::error::AppError;
use crate::models::subtask::{CreateSubtask, Subtask, UpdateSubtask};
use crate::repository::subtask_repo;

fn lock_db(db: &Mutex<Connection>) -> Result<std::sync::MutexGuard<'_, Connection>, AppError> {
  db.lock().map_err(|e| {
    log::error!("Mutex poisoned: {e}");
    AppError::LockError
  })
}

#[tauri::command]
pub fn get_subtasks_by_task(
  db: State<Mutex<Connection>>,
  task_uuid: String,
) -> Result<Vec<Subtask>, AppError> {
  let conn = lock_db(&db)?;
  subtask_repo::get_by_task(&conn, &task_uuid)
}

#[tauri::command]
pub fn create_subtask(
  db: State<Mutex<Connection>>,
  data: CreateSubtask,
) -> Result<Subtask, AppError> {
  if data.name.trim().is_empty() {
    return Err(AppError::Validation("Имя подзадачи не может быть пустым".into()));
  }
  if data.name.len() > 255 {
    return Err(AppError::Validation("Имя подзадачи слишком длинное (макс. 255)".into()));
  }
  let conn = lock_db(&db)?;
  subtask_repo::create(&conn, data)
}

#[tauri::command]
pub fn update_subtask(
  db: State<Mutex<Connection>>,
  uuid: String,
  data: UpdateSubtask,
) -> Result<Subtask, AppError> {
  if let Some(name) = &data.name {
    if name.trim().is_empty() {
      return Err(AppError::Validation("Имя подзадачи не может быть пустым".into()));
    }
    if name.len() > 255 {
      return Err(AppError::Validation("Имя подзадачи слишком длинное (макс. 255)".into()));
    }
  }
  let conn = lock_db(&db)?;
  subtask_repo::update(&conn, &uuid, data)
}

#[tauri::command]
pub fn delete_subtask(db: State<Mutex<Connection>>, uuid: String) -> Result<(), AppError> {
  let conn = lock_db(&db)?;
  subtask_repo::delete(&conn, &uuid)
}
