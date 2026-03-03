use rusqlite::{params, Connection};
use uuid::Uuid;

use crate::error::AppError;
use crate::models::subtask::{CreateSubtask, Subtask, UpdateSubtask};
use crate::repository::project_repo::now;

pub fn get_by_task(conn: &Connection, task_uuid: &str) -> Result<Vec<Subtask>, AppError> {
  let mut stmt = conn.prepare(
    "SELECT uuid, task_uuid, name, is_completed, position, created_at, updated_at
     FROM subtasks WHERE task_uuid = ?1 ORDER BY position ASC",
  )?;

  let rows = stmt.query_map(params![task_uuid], |row| {
    Ok(Subtask {
      uuid: row.get(0)?,
      task_uuid: row.get(1)?,
      name: row.get(2)?,
      is_completed: row.get::<_, i32>(3)? != 0,
      position: row.get(4)?,
      created_at: row.get(5)?,
      updated_at: row.get(6)?,
    })
  })?;

  let mut subtasks = Vec::new();
  for row in rows {
    subtasks.push(row?);
  }
  Ok(subtasks)
}

pub fn create(conn: &Connection, data: CreateSubtask) -> Result<Subtask, AppError> {
  let id = Uuid::new_v4().to_string();
  let now = now();

  let max_pos: i32 = conn
    .query_row(
      "SELECT COALESCE(MAX(position), -1) FROM subtasks WHERE task_uuid = ?1",
      params![data.task_uuid],
      |row| row.get(0),
    )
    .unwrap_or(-1);

  conn.execute(
    "INSERT INTO subtasks (uuid, task_uuid, name, is_completed, position, created_at, updated_at)
     VALUES (?1, ?2, ?3, 0, ?4, ?5, ?6)",
    params![id, data.task_uuid, data.name, max_pos + 1, now, now],
  )?;

  Ok(Subtask {
    uuid: id,
    task_uuid: data.task_uuid,
    name: data.name,
    is_completed: false,
    position: max_pos + 1,
    created_at: now.clone(),
    updated_at: now,
  })
}

pub fn update(conn: &Connection, uuid: &str, data: UpdateSubtask) -> Result<Subtask, AppError> {
  let exists: bool = conn.query_row(
    "SELECT COUNT(*) > 0 FROM subtasks WHERE uuid = ?1",
    params![uuid],
    |row| row.get(0),
  )?;
  if !exists {
    return Err(AppError::NotFound(format!("Подзадача {uuid} не найдена")));
  }

  let now = now();

  if let Some(name) = &data.name {
    conn.execute(
      "UPDATE subtasks SET name = ?1, updated_at = ?2 WHERE uuid = ?3",
      params![name, now, uuid],
    )?;
  }
  if let Some(is_completed) = &data.is_completed {
    let val: i32 = if *is_completed { 1 } else { 0 };
    conn.execute(
      "UPDATE subtasks SET is_completed = ?1, updated_at = ?2 WHERE uuid = ?3",
      params![val, now, uuid],
    )?;
  }
  if let Some(position) = &data.position {
    conn.execute(
      "UPDATE subtasks SET position = ?1, updated_at = ?2 WHERE uuid = ?3",
      params![position, now, uuid],
    )?;
  }

  get_by_uuid(conn, uuid)
}

pub fn delete(conn: &Connection, uuid: &str) -> Result<(), AppError> {
  let affected = conn.execute("DELETE FROM subtasks WHERE uuid = ?1", params![uuid])?;
  if affected == 0 {
    return Err(AppError::NotFound(format!("Подзадача {uuid} не найдена")));
  }
  Ok(())
}

fn get_by_uuid(conn: &Connection, uuid: &str) -> Result<Subtask, AppError> {
  conn
    .query_row(
      "SELECT uuid, task_uuid, name, is_completed, position, created_at, updated_at
       FROM subtasks WHERE uuid = ?1",
      params![uuid],
      |row| {
        Ok(Subtask {
          uuid: row.get(0)?,
          task_uuid: row.get(1)?,
          name: row.get(2)?,
          is_completed: row.get::<_, i32>(3)? != 0,
          position: row.get(4)?,
          created_at: row.get(5)?,
          updated_at: row.get(6)?,
        })
      },
    )
    .map_err(|_| AppError::NotFound(format!("Подзадача {uuid} не найдена")))
}
