use rusqlite::{params, Connection};
use uuid::Uuid;

use crate::error::AppError;
use crate::models::priority::{CreatePriority, Priority, UpdatePriority};
use crate::utils;

/// Внутренний метод для создания дефолтных приоритетов
pub fn create_internal(
  conn: &Connection,
  project_uuid: &str,
  name: &str,
  color: &str,
  level: i32,
) -> Result<Priority, AppError> {
  let id = Uuid::new_v4().to_string();
  let now = utils::now_utc();

  conn.execute(
    "INSERT INTO priorities (uuid, project_uuid, name, color, level, created_at, updated_at)
     VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
    params![id, project_uuid, name, color, level, now, now],
  )?;

  Ok(Priority {
    uuid: id,
    project_uuid: project_uuid.to_string(),
    name: name.to_string(),
    color: color.to_string(),
    level,
    created_at: now.clone(),
    updated_at: now,
  })
}

pub fn get_by_project(conn: &Connection, project_uuid: &str) -> Result<Vec<Priority>, AppError> {
  let mut stmt = conn.prepare(
    "SELECT uuid, project_uuid, name, color, level, created_at, updated_at
     FROM priorities WHERE project_uuid = ?1 ORDER BY level ASC",
  )?;

  let rows = stmt.query_map(params![project_uuid], |row| {
    Ok(Priority {
      uuid: row.get(0)?,
      project_uuid: row.get(1)?,
      name: row.get(2)?,
      color: row.get(3)?,
      level: row.get(4)?,
      created_at: row.get(5)?,
      updated_at: row.get(6)?,
    })
  })?;

  let mut priorities = Vec::new();
  for row in rows {
    priorities.push(row?);
  }
  Ok(priorities)
}

pub fn create(conn: &Connection, data: CreatePriority) -> Result<Priority, AppError> {
  create_internal(conn, &data.project_uuid, &data.name, &data.color, data.level)
}

pub fn update(conn: &Connection, uuid: &str, data: UpdatePriority) -> Result<Priority, AppError> {
  let exists: bool = conn.query_row(
    "SELECT COUNT(*) > 0 FROM priorities WHERE uuid = ?1",
    params![uuid],
    |row| row.get(0),
  )?;
  if !exists {
    return Err(AppError::NotFound(format!("Приоритет {uuid} не найден")));
  }

  let now = utils::now_utc();

  if let Some(name) = &data.name {
    conn.execute(
      "UPDATE priorities SET name = ?1, updated_at = ?2 WHERE uuid = ?3",
      params![name, now, uuid],
    )?;
  }
  if let Some(color) = &data.color {
    conn.execute(
      "UPDATE priorities SET color = ?1, updated_at = ?2 WHERE uuid = ?3",
      params![color, now, uuid],
    )?;
  }
  if let Some(level) = &data.level {
    conn.execute(
      "UPDATE priorities SET level = ?1, updated_at = ?2 WHERE uuid = ?3",
      params![level, now, uuid],
    )?;
  }

  get_by_uuid(conn, uuid)
}

pub fn delete(conn: &Connection, uuid: &str) -> Result<(), AppError> {
  let affected = conn.execute("DELETE FROM priorities WHERE uuid = ?1", params![uuid])?;
  if affected == 0 {
    return Err(AppError::NotFound(format!("Приоритет {uuid} не найден")));
  }
  Ok(())
}

fn get_by_uuid(conn: &Connection, uuid: &str) -> Result<Priority, AppError> {
  conn
    .query_row(
      "SELECT uuid, project_uuid, name, color, level, created_at, updated_at
       FROM priorities WHERE uuid = ?1",
      params![uuid],
      |row| {
        Ok(Priority {
          uuid: row.get(0)?,
          project_uuid: row.get(1)?,
          name: row.get(2)?,
          color: row.get(3)?,
          level: row.get(4)?,
          created_at: row.get(5)?,
          updated_at: row.get(6)?,
        })
      },
    )
    .map_err(|_| AppError::NotFound(format!("Приоритет {uuid} не найден")))
}
