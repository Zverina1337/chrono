use rusqlite::{params, Connection};
use uuid::Uuid;

use crate::error::AppError;
use crate::models::status::{CreateStatus, Status, UpdateStatus};
use crate::repository::project_repo::now;

/// Внутренний метод для создания дефолтных статусов (без генерации позиции)
pub fn create_internal(
  conn: &Connection,
  project_uuid: &str,
  name: &str,
  color: &str,
  position: i32,
) -> Result<Status, AppError> {
  let id = Uuid::new_v4().to_string();
  let now = now();

  conn.execute(
    "INSERT INTO statuses (uuid, project_uuid, name, color, position, created_at, updated_at)
     VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
    params![id, project_uuid, name, color, position, now, now],
  )?;

  Ok(Status {
    uuid: id,
    project_uuid: project_uuid.to_string(),
    name: name.to_string(),
    color: color.to_string(),
    position,
    created_at: now.clone(),
    updated_at: now,
  })
}

pub fn get_by_project(conn: &Connection, project_uuid: &str) -> Result<Vec<Status>, AppError> {
  let mut stmt = conn.prepare(
    "SELECT uuid, project_uuid, name, color, position, created_at, updated_at
     FROM statuses WHERE project_uuid = ?1 ORDER BY position ASC",
  )?;

  let rows = stmt.query_map(params![project_uuid], |row| {
    Ok(Status {
      uuid: row.get(0)?,
      project_uuid: row.get(1)?,
      name: row.get(2)?,
      color: row.get(3)?,
      position: row.get(4)?,
      created_at: row.get(5)?,
      updated_at: row.get(6)?,
    })
  })?;

  let mut statuses = Vec::new();
  for row in rows {
    statuses.push(row?);
  }
  Ok(statuses)
}

pub fn create(conn: &Connection, data: CreateStatus) -> Result<Status, AppError> {
  let max_pos: i32 = conn
    .query_row(
      "SELECT COALESCE(MAX(position), -1) FROM statuses WHERE project_uuid = ?1",
      params![data.project_uuid],
      |row| row.get(0),
    )
    .unwrap_or(-1);

  create_internal(conn, &data.project_uuid, &data.name, &data.color, max_pos + 1)
}

pub fn update(conn: &Connection, uuid: &str, data: UpdateStatus) -> Result<Status, AppError> {
  let exists: bool = conn.query_row(
    "SELECT COUNT(*) > 0 FROM statuses WHERE uuid = ?1",
    params![uuid],
    |row| row.get(0),
  )?;
  if !exists {
    return Err(AppError::NotFound(format!("Статус {uuid} не найден")));
  }

  let now = now();

  if let Some(name) = &data.name {
    conn.execute(
      "UPDATE statuses SET name = ?1, updated_at = ?2 WHERE uuid = ?3",
      params![name, now, uuid],
    )?;
  }
  if let Some(color) = &data.color {
    conn.execute(
      "UPDATE statuses SET color = ?1, updated_at = ?2 WHERE uuid = ?3",
      params![color, now, uuid],
    )?;
  }
  if let Some(position) = &data.position {
    conn.execute(
      "UPDATE statuses SET position = ?1, updated_at = ?2 WHERE uuid = ?3",
      params![position, now, uuid],
    )?;
  }

  get_by_uuid(conn, uuid)
}

pub fn delete(conn: &Connection, uuid: &str) -> Result<(), AppError> {
  let affected = conn.execute("DELETE FROM statuses WHERE uuid = ?1", params![uuid])?;
  if affected == 0 {
    return Err(AppError::NotFound(format!("Статус {uuid} не найден")));
  }
  Ok(())
}

fn get_by_uuid(conn: &Connection, uuid: &str) -> Result<Status, AppError> {
  conn
    .query_row(
      "SELECT uuid, project_uuid, name, color, position, created_at, updated_at
       FROM statuses WHERE uuid = ?1",
      params![uuid],
      |row| {
        Ok(Status {
          uuid: row.get(0)?,
          project_uuid: row.get(1)?,
          name: row.get(2)?,
          color: row.get(3)?,
          position: row.get(4)?,
          created_at: row.get(5)?,
          updated_at: row.get(6)?,
        })
      },
    )
    .map_err(|_| AppError::NotFound(format!("Статус {uuid} не найден")))
}
