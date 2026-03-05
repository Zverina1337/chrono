use rusqlite::{params, Connection};
use uuid::Uuid;

use crate::error::AppError;
use crate::models::task::{CreateTask, Task, UpdateTask};
use crate::utils;

fn row_to_task(row: &rusqlite::Row) -> rusqlite::Result<Task> {
  Ok(Task {
    uuid: row.get(0)?,
    project_uuid: row.get(1)?,
    status_uuid: row.get(2)?,
    priority_uuid: row.get(3)?,
    name: row.get(4)?,
    description: row.get(5)?,
    due_date: row.get(6)?,
    start_date: row.get(7)?,
    estimated_seconds: row.get(8)?,
    position: row.get(9)?,
    created_at: row.get(10)?,
    updated_at: row.get(11)?,
  })
}

const SELECT_COLS: &str =
  "uuid, project_uuid, status_uuid, priority_uuid, name, description,
   due_date, start_date, estimated_seconds, position, created_at, updated_at";

pub fn get_by_project(conn: &Connection, project_uuid: &str) -> Result<Vec<Task>, AppError> {
  let sql = format!(
    "SELECT {SELECT_COLS} FROM tasks WHERE project_uuid = ?1 ORDER BY position ASC, created_at ASC"
  );
  let mut stmt = conn.prepare(&sql)?;

  let rows = stmt.query_map(params![project_uuid], row_to_task)?;

  let mut tasks = Vec::new();
  for row in rows {
    tasks.push(row?);
  }
  Ok(tasks)
}

pub fn get_by_uuid(conn: &Connection, uuid: &str) -> Result<Task, AppError> {
  let sql = format!("SELECT {SELECT_COLS} FROM tasks WHERE uuid = ?1");
  conn
    .query_row(&sql, params![uuid], row_to_task)
    .map_err(|_| AppError::NotFound(format!("Задача {uuid} не найдена")))
}

pub fn create(conn: &Connection, data: CreateTask) -> Result<Task, AppError> {
  let id = Uuid::new_v4().to_string();
  let now = utils::now_utc();
  let position = utils::next_position(conn, "tasks", "project_uuid", &data.project_uuid)?;

  conn.execute(
    "INSERT INTO tasks (uuid, project_uuid, status_uuid, priority_uuid, name, description,
     due_date, start_date, estimated_seconds, position, created_at, updated_at)
     VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
    params![
      id,
      data.project_uuid,
      data.status_uuid,
      data.priority_uuid,
      data.name,
      data.description,
      data.due_date,
      data.start_date,
      data.estimated_seconds,
      position,
      now,
      now
    ],
  )?;

  get_by_uuid(conn, &id)
}

pub fn update(conn: &Connection, uuid: &str, data: UpdateTask) -> Result<Task, AppError> {
  let exists: bool = conn.query_row(
    "SELECT COUNT(*) > 0 FROM tasks WHERE uuid = ?1",
    params![uuid],
    |row| row.get(0),
  )?;
  if !exists {
    return Err(AppError::NotFound(format!("Задача {uuid} не найдена")));
  }

  let now = utils::now_utc();

  if let Some(status_uuid) = &data.status_uuid {
    conn.execute(
      "UPDATE tasks SET status_uuid = ?1, updated_at = ?2 WHERE uuid = ?3",
      params![status_uuid, now, uuid],
    )?;
  }
  if let Some(priority_uuid) = &data.priority_uuid {
    conn.execute(
      "UPDATE tasks SET priority_uuid = ?1, updated_at = ?2 WHERE uuid = ?3",
      params![priority_uuid, now, uuid],
    )?;
  }
  if let Some(name) = &data.name {
    conn.execute(
      "UPDATE tasks SET name = ?1, updated_at = ?2 WHERE uuid = ?3",
      params![name, now, uuid],
    )?;
  }
  if let Some(description) = &data.description {
    conn.execute(
      "UPDATE tasks SET description = ?1, updated_at = ?2 WHERE uuid = ?3",
      params![description, now, uuid],
    )?;
  }
  if let Some(due_date) = &data.due_date {
    conn.execute(
      "UPDATE tasks SET due_date = ?1, updated_at = ?2 WHERE uuid = ?3",
      params![due_date, now, uuid],
    )?;
  }
  if let Some(start_date) = &data.start_date {
    conn.execute(
      "UPDATE tasks SET start_date = ?1, updated_at = ?2 WHERE uuid = ?3",
      params![start_date, now, uuid],
    )?;
  }
  if let Some(estimated_seconds) = &data.estimated_seconds {
    conn.execute(
      "UPDATE tasks SET estimated_seconds = ?1, updated_at = ?2 WHERE uuid = ?3",
      params![estimated_seconds, now, uuid],
    )?;
  }
  if let Some(position) = &data.position {
    conn.execute(
      "UPDATE tasks SET position = ?1, updated_at = ?2 WHERE uuid = ?3",
      params![position, now, uuid],
    )?;
  }

  get_by_uuid(conn, uuid)
}

pub fn delete(conn: &Connection, uuid: &str) -> Result<(), AppError> {
  let affected = conn.execute("DELETE FROM tasks WHERE uuid = ?1", params![uuid])?;
  if affected == 0 {
    return Err(AppError::NotFound(format!("Задача {uuid} не найдена")));
  }
  Ok(())
}
