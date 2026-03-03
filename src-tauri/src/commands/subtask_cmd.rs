use std::sync::Mutex;

use rusqlite::Connection;
use tauri::State;

use crate::error::AppError;
use crate::models::subtask::{CreateSubtask, Subtask, UpdateSubtask};
use crate::repository::subtask_repo;

#[tauri::command]
pub fn get_subtasks_by_task(
  db: State<Mutex<Connection>>,
  task_uuid: String,
) -> Result<Vec<Subtask>, AppError> {
  let conn = db.lock().unwrap();
  subtask_repo::get_by_task(&conn, &task_uuid)
}

#[tauri::command]
pub fn create_subtask(
  db: State<Mutex<Connection>>,
  data: CreateSubtask,
) -> Result<Subtask, AppError> {
  let conn = db.lock().unwrap();
  subtask_repo::create(&conn, data)
}

#[tauri::command]
pub fn update_subtask(
  db: State<Mutex<Connection>>,
  uuid: String,
  data: UpdateSubtask,
) -> Result<Subtask, AppError> {
  let conn = db.lock().unwrap();
  subtask_repo::update(&conn, &uuid, data)
}

#[tauri::command]
pub fn delete_subtask(db: State<Mutex<Connection>>, uuid: String) -> Result<(), AppError> {
  let conn = db.lock().unwrap();
  subtask_repo::delete(&conn, &uuid)
}
