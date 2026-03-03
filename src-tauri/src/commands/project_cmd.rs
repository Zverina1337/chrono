use std::sync::Mutex;

use rusqlite::Connection;
use tauri::State;

use crate::error::AppError;
use crate::models::project::{CreateProject, Project, UpdateProject};
use crate::repository::project_repo;

#[tauri::command]
pub fn get_all_projects(db: State<Mutex<Connection>>) -> Result<Vec<Project>, AppError> {
  let conn = db.lock().unwrap();
  project_repo::get_all(&conn)
}

#[tauri::command]
pub fn create_project(
  db: State<Mutex<Connection>>,
  data: CreateProject,
) -> Result<Project, AppError> {
  let conn = db.lock().unwrap();
  project_repo::create(&conn, data)
}

#[tauri::command]
pub fn update_project(
  db: State<Mutex<Connection>>,
  uuid: String,
  data: UpdateProject,
) -> Result<Project, AppError> {
  let conn = db.lock().unwrap();
  project_repo::update(&conn, &uuid, data)
}

#[tauri::command]
pub fn delete_project(db: State<Mutex<Connection>>, uuid: String) -> Result<(), AppError> {
  let conn = db.lock().unwrap();
  project_repo::delete(&conn, &uuid)
}
