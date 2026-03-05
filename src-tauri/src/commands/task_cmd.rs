use std::sync::Mutex;

use rusqlite::Connection;
use tauri::State;

use crate::error::AppError;
use crate::models::task::{CreateTask, Task, UpdateTask};
use crate::repository::task_repo;

fn lock_db(db: &Mutex<Connection>) -> Result<std::sync::MutexGuard<'_, Connection>, AppError> {
  db.lock().map_err(|e| {
    log::error!("Mutex poisoned: {e}");
    AppError::LockError
  })
}

#[tauri::command]
pub fn get_tasks_by_project(
  db: State<Mutex<Connection>>,
  project_uuid: String,
) -> Result<Vec<Task>, AppError> {
  let conn = lock_db(&db)?;
  task_repo::get_by_project(&conn, &project_uuid)
}

#[tauri::command]
pub fn get_task(db: State<Mutex<Connection>>, uuid: String) -> Result<Task, AppError> {
  let conn = lock_db(&db)?;
  task_repo::get_by_uuid(&conn, &uuid)
}

#[tauri::command]
pub fn create_task(db: State<Mutex<Connection>>, data: CreateTask) -> Result<Task, AppError> {
  if data.name.trim().is_empty() {
    return Err(AppError::Validation("Имя задачи не может быть пустым".into()));
  }
  if data.name.len() > 255 {
    return Err(AppError::Validation("Имя задачи слишком длинное (макс. 255)".into()));
  }
  let conn = lock_db(&db)?;
  task_repo::create(&conn, data)
}

#[tauri::command]
pub fn update_task(
  db: State<Mutex<Connection>>,
  uuid: String,
  data: UpdateTask,
) -> Result<Task, AppError> {
  if let Some(name) = &data.name {
    if name.trim().is_empty() {
      return Err(AppError::Validation("Имя задачи не может быть пустым".into()));
    }
    if name.len() > 255 {
      return Err(AppError::Validation("Имя задачи слишком длинное (макс. 255)".into()));
    }
  }
  let conn = lock_db(&db)?;
  task_repo::update(&conn, &uuid, data)
}

#[tauri::command]
pub fn delete_task(db: State<Mutex<Connection>>, uuid: String) -> Result<(), AppError> {
  let conn = lock_db(&db)?;
  task_repo::delete(&conn, &uuid)
}
