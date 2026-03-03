use rusqlite::{params, Connection};
use uuid::Uuid;

use crate::error::AppError;
use crate::models::time_entry::{CreateTimeEntry, TimeEntry, UpdateTimeEntry};
use crate::repository::project_repo::now;

fn row_to_entry(row: &rusqlite::Row) -> rusqlite::Result<TimeEntry> {
  Ok(TimeEntry {
    uuid: row.get(0)?,
    task_uuid: row.get(1)?,
    started_at: row.get(2)?,
    ended_at: row.get(3)?,
    duration_minutes: row.get(4)?,
    description: row.get(5)?,
    created_at: row.get(6)?,
  })
}

const SELECT_COLS: &str =
  "uuid, task_uuid, started_at, ended_at, duration_minutes, description, created_at";

pub fn get_by_task(conn: &Connection, task_uuid: &str) -> Result<Vec<TimeEntry>, AppError> {
  let sql = format!(
    "SELECT {SELECT_COLS} FROM time_entries WHERE task_uuid = ?1 ORDER BY started_at DESC"
  );
  let mut stmt = conn.prepare(&sql)?;

  let rows = stmt.query_map(params![task_uuid], row_to_entry)?;

  let mut entries = Vec::new();
  for row in rows {
    entries.push(row?);
  }
  Ok(entries)
}

pub fn create(conn: &Connection, data: CreateTimeEntry) -> Result<TimeEntry, AppError> {
  let id = Uuid::new_v4().to_string();
  let now = now();

  conn.execute(
    "INSERT INTO time_entries (uuid, task_uuid, started_at, ended_at, duration_minutes, description, created_at)
     VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
    params![
      id,
      data.task_uuid,
      data.started_at,
      data.ended_at,
      data.duration_minutes,
      data.description,
      now
    ],
  )?;

  Ok(TimeEntry {
    uuid: id,
    task_uuid: data.task_uuid,
    started_at: data.started_at,
    ended_at: data.ended_at,
    duration_minutes: data.duration_minutes,
    description: data.description,
    created_at: now,
  })
}

pub fn update(
  conn: &Connection,
  uuid: &str,
  data: UpdateTimeEntry,
) -> Result<TimeEntry, AppError> {
  let exists: bool = conn.query_row(
    "SELECT COUNT(*) > 0 FROM time_entries WHERE uuid = ?1",
    params![uuid],
    |row| row.get(0),
  )?;
  if !exists {
    return Err(AppError::NotFound(format!(
      "Запись времени {uuid} не найдена"
    )));
  }

  if let Some(ended_at) = &data.ended_at {
    conn.execute(
      "UPDATE time_entries SET ended_at = ?1 WHERE uuid = ?2",
      params![ended_at, uuid],
    )?;
  }
  if let Some(duration_minutes) = &data.duration_minutes {
    conn.execute(
      "UPDATE time_entries SET duration_minutes = ?1 WHERE uuid = ?2",
      params![duration_minutes, uuid],
    )?;
  }
  if let Some(description) = &data.description {
    conn.execute(
      "UPDATE time_entries SET description = ?1 WHERE uuid = ?2",
      params![description, uuid],
    )?;
  }

  get_by_uuid(conn, uuid)
}

pub fn delete(conn: &Connection, uuid: &str) -> Result<(), AppError> {
  let affected = conn.execute("DELETE FROM time_entries WHERE uuid = ?1", params![uuid])?;
  if affected == 0 {
    return Err(AppError::NotFound(format!(
      "Запись времени {uuid} не найдена"
    )));
  }
  Ok(())
}

fn get_by_uuid(conn: &Connection, uuid: &str) -> Result<TimeEntry, AppError> {
  let sql = format!("SELECT {SELECT_COLS} FROM time_entries WHERE uuid = ?1");
  conn
    .query_row(&sql, params![uuid], row_to_entry)
    .map_err(|_| AppError::NotFound(format!("Запись времени {uuid} не найдена")))
}
