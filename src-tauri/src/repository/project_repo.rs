use rusqlite::{params, Connection};
use uuid::Uuid;

use crate::error::AppError;
use crate::models::project::{CreateProject, Project, UpdateProject};
use crate::utils;

/// Дефолтные статусы для нового проекта: (name, color, position)
const DEFAULT_STATUSES: &[(&str, &str, i32)] = &[
  ("К работе", "#6366f1", 0),
  ("В работе", "#f59e0b", 1),
  ("Сделано", "#22c55e", 2),
];

/// Дефолтные приоритеты для нового проекта: (name, color, level)
const DEFAULT_PRIORITIES: &[(&str, &str, i32)] = &[
  ("Низкий", "#22c55e", 0),
  ("Средний", "#eab308", 1),
  ("Высокий", "#f97316", 2),
  ("Срочный", "#ef4444", 3),
];

pub fn get_all(conn: &Connection) -> Result<Vec<Project>, AppError> {
  let mut stmt = conn.prepare(
    "SELECT uuid, name, description, position, created_at, updated_at
     FROM projects ORDER BY position ASC, created_at ASC",
  )?;

  let rows = stmt.query_map([], |row| {
    Ok(Project {
      uuid: row.get(0)?,
      name: row.get(1)?,
      description: row.get(2)?,
      position: row.get(3)?,
      created_at: row.get(4)?,
      updated_at: row.get(5)?,
    })
  })?;

  let mut projects = Vec::new();
  for row in rows {
    projects.push(row?);
  }
  Ok(projects)
}

pub fn create(conn: &Connection, data: CreateProject) -> Result<Project, AppError> {
  let id = Uuid::new_v4().to_string();
  let now = utils::now_utc();
  let position = utils::next_position_global(conn, "projects")?;

  let tx = conn.unchecked_transaction()?;

  tx.execute(
    "INSERT INTO projects (uuid, name, description, position, created_at, updated_at)
     VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
    params![id, data.name, data.description, position, now, now],
  )?;

  create_defaults(&tx, &id, &now)?;

  tx.commit()?;

  Ok(Project {
    uuid: id,
    name: data.name,
    description: data.description,
    position,
    created_at: now.clone(),
    updated_at: now,
  })
}

/// Создаёт дефолтные статусы и приоритеты для проекта
fn create_defaults(conn: &Connection, project_uuid: &str, now: &str) -> Result<(), AppError> {
  for (name, color, position) in DEFAULT_STATUSES {
    let id = Uuid::new_v4().to_string();
    conn.execute(
      "INSERT INTO statuses (uuid, project_uuid, name, color, position, created_at, updated_at)
       VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
      params![id, project_uuid, name, color, position, now, now],
    )?;
  }

  for (name, color, level) in DEFAULT_PRIORITIES {
    let id = Uuid::new_v4().to_string();
    conn.execute(
      "INSERT INTO priorities (uuid, project_uuid, name, color, level, created_at, updated_at)
       VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
      params![id, project_uuid, name, color, level, now, now],
    )?;
  }

  Ok(())
}

pub fn update(conn: &Connection, uuid: &str, data: UpdateProject) -> Result<Project, AppError> {
  let exists: bool = conn.query_row(
    "SELECT COUNT(*) > 0 FROM projects WHERE uuid = ?1",
    params![uuid],
    |row| row.get(0),
  )?;
  if !exists {
    return Err(AppError::NotFound(format!("Проект {uuid} не найден")));
  }

  let now = utils::now_utc();

  if let Some(name) = &data.name {
    conn.execute(
      "UPDATE projects SET name = ?1, updated_at = ?2 WHERE uuid = ?3",
      params![name, now, uuid],
    )?;
  }
  if let Some(description) = &data.description {
    conn.execute(
      "UPDATE projects SET description = ?1, updated_at = ?2 WHERE uuid = ?3",
      params![description, now, uuid],
    )?;
  }
  if let Some(position) = &data.position {
    conn.execute(
      "UPDATE projects SET position = ?1, updated_at = ?2 WHERE uuid = ?3",
      params![position, now, uuid],
    )?;
  }

  get_by_uuid(conn, uuid)
}

pub fn delete(conn: &Connection, uuid: &str) -> Result<(), AppError> {
  let affected = conn.execute("DELETE FROM projects WHERE uuid = ?1", params![uuid])?;
  if affected == 0 {
    return Err(AppError::NotFound(format!("Проект {uuid} не найден")));
  }
  Ok(())
}

pub fn get_by_uuid(conn: &Connection, uuid: &str) -> Result<Project, AppError> {
  conn
    .query_row(
      "SELECT uuid, name, description, position, created_at, updated_at
       FROM projects WHERE uuid = ?1",
      params![uuid],
      |row| {
        Ok(Project {
          uuid: row.get(0)?,
          name: row.get(1)?,
          description: row.get(2)?,
          position: row.get(3)?,
          created_at: row.get(4)?,
          updated_at: row.get(5)?,
        })
      },
    )
    .map_err(|_| AppError::NotFound(format!("Проект {uuid} не найден")))
}
