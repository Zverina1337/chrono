use rusqlite::{params, Connection};
use uuid::Uuid;

use crate::error::AppError;
use crate::models::label::{CreateLabel, Label};

pub fn get_all(conn: &Connection) -> Result<Vec<Label>, AppError> {
  let mut stmt = conn.prepare("SELECT uuid, name, color FROM labels ORDER BY name ASC")?;

  let rows = stmt.query_map([], |row| {
    Ok(Label {
      uuid: row.get(0)?,
      name: row.get(1)?,
      color: row.get(2)?,
    })
  })?;

  let mut labels = Vec::new();
  for row in rows {
    labels.push(row?);
  }
  Ok(labels)
}

pub fn create(conn: &Connection, data: CreateLabel) -> Result<Label, AppError> {
  let id = Uuid::new_v4().to_string();

  conn.execute(
    "INSERT INTO labels (uuid, name, color) VALUES (?1, ?2, ?3)",
    params![id, data.name, data.color],
  )?;

  Ok(Label {
    uuid: id,
    name: data.name,
    color: data.color,
  })
}

pub fn delete(conn: &Connection, uuid: &str) -> Result<(), AppError> {
  let affected = conn.execute("DELETE FROM labels WHERE uuid = ?1", params![uuid])?;
  if affected == 0 {
    return Err(AppError::NotFound(format!("Лейбл {uuid} не найден")));
  }
  Ok(())
}

pub fn add_to_task(conn: &Connection, task_uuid: &str, label_uuid: &str) -> Result<(), AppError> {
  conn.execute(
    "INSERT OR IGNORE INTO task_labels (task_uuid, label_uuid) VALUES (?1, ?2)",
    params![task_uuid, label_uuid],
  )?;
  Ok(())
}

pub fn remove_from_task(
  conn: &Connection,
  task_uuid: &str,
  label_uuid: &str,
) -> Result<(), AppError> {
  conn.execute(
    "DELETE FROM task_labels WHERE task_uuid = ?1 AND label_uuid = ?2",
    params![task_uuid, label_uuid],
  )?;
  Ok(())
}
