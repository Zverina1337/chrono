use std::sync::Mutex;

use rusqlite::Connection;
use tauri::State;

use crate::error::AppError;
use crate::models::project::{CreateProject, Project, UpdateProject};
use crate::repository::project_repo;

fn lock_db(db: &Mutex<Connection>) -> Result<std::sync::MutexGuard<'_, Connection>, AppError> {
  db.lock().map_err(|e| {
    log::error!("Mutex poisoned: {e}");
    AppError::LockError
  })
}

#[tauri::command]
pub fn get_all_projects(db: State<Mutex<Connection>>) -> Result<Vec<Project>, AppError> {
  let conn = lock_db(&db)?;
  project_repo::get_all(&conn)
}

#[tauri::command]
pub fn create_project(
  db: State<Mutex<Connection>>,
  data: CreateProject,
) -> Result<Project, AppError> {
  if data.name.trim().is_empty() {
    return Err(AppError::Validation("Имя проекта не может быть пустым".into()));
  }
  if data.name.len() > 255 {
    return Err(AppError::Validation("Имя проекта слишком длинное (макс. 255)".into()));
  }
  let conn = lock_db(&db)?;
  project_repo::create(&conn, data)
}

#[tauri::command]
pub fn update_project(
  db: State<Mutex<Connection>>,
  uuid: String,
  data: UpdateProject,
) -> Result<Project, AppError> {
  if let Some(name) = &data.name {
    if name.trim().is_empty() {
      return Err(AppError::Validation("Имя проекта не может быть пустым".into()));
    }
    if name.len() > 255 {
      return Err(AppError::Validation("Имя проекта слишком длинное (макс. 255)".into()));
    }
  }
  let conn = lock_db(&db)?;
  project_repo::update(&conn, &uuid, data)
}

#[tauri::command]
pub fn delete_project(db: State<Mutex<Connection>>, uuid: String) -> Result<(), AppError> {
  let conn = lock_db(&db)?;
  project_repo::delete(&conn, &uuid)
}
