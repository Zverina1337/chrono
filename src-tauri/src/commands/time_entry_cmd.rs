use std::sync::Mutex;

use rusqlite::Connection;
use tauri::State;

use crate::error::AppError;
use crate::models::time_entry::{CreateTimeEntry, TimeEntry, UpdateTimeEntry};
use crate::repository::time_entry_repo;

#[tauri::command]
pub fn get_time_entries_by_task(
  db: State<Mutex<Connection>>,
  task_uuid: String,
) -> Result<Vec<TimeEntry>, AppError> {
  let conn = db.lock().unwrap();
  time_entry_repo::get_by_task(&conn, &task_uuid)
}

#[tauri::command]
pub fn create_time_entry(
  db: State<Mutex<Connection>>,
  data: CreateTimeEntry,
) -> Result<TimeEntry, AppError> {
  let conn = db.lock().unwrap();
  time_entry_repo::create(&conn, data)
}

#[tauri::command]
pub fn update_time_entry(
  db: State<Mutex<Connection>>,
  uuid: String,
  data: UpdateTimeEntry,
) -> Result<TimeEntry, AppError> {
  let conn = db.lock().unwrap();
  time_entry_repo::update(&conn, &uuid, data)
}

#[tauri::command]
pub fn delete_time_entry(db: State<Mutex<Connection>>, uuid: String) -> Result<(), AppError> {
  let conn = db.lock().unwrap();
  time_entry_repo::delete(&conn, &uuid)
}
