use rusqlite::{params, Connection};
use uuid::Uuid;

use crate::error::AppError;
use crate::models::project::{CreateProject, Project, UpdateProject};
use crate::repository::priority_repo;
use crate::repository::status_repo;

/// Текущее UTC-время в ISO 8601 формате
pub(crate) fn now() -> String {
  use std::time::SystemTime;
  let duration = SystemTime::now()
    .duration_since(SystemTime::UNIX_EPOCH)
    .unwrap();
  let secs = duration.as_secs();
  // Простое форматирование UTC без внешних крейтов
  let days = secs / 86400;
  let time_secs = secs % 86400;
  let hours = time_secs / 3600;
  let minutes = (time_secs % 3600) / 60;
  let seconds = time_secs % 60;

  // Вычисление даты из дней с эпохи Unix
  let (year, month, day) = days_to_date(days as i64);
  format!(
    "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
    year, month, day, hours, minutes, seconds
  )
}

fn days_to_date(days: i64) -> (i64, i64, i64) {
  // Алгоритм из http://howardhinnant.github.io/date_algorithms.html
  let z = days + 719468;
  let era = if z >= 0 { z } else { z - 146096 } / 146097;
  let doe = z - era * 146097;
  let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
  let y = yoe + era * 400;
  let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
  let mp = (5 * doy + 2) / 153;
  let d = doy - (153 * mp + 2) / 5 + 1;
  let m = if mp < 10 { mp + 3 } else { mp - 9 };
  let y = if m <= 2 { y + 1 } else { y };
  (y, m, d)
}

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
  let now = now();

  // Получаем максимальную позицию
  let max_pos: i32 = conn
    .query_row(
      "SELECT COALESCE(MAX(position), -1) FROM projects",
      [],
      |row| row.get(0),
    )
    .unwrap_or(-1);

  conn.execute(
    "INSERT INTO projects (uuid, name, description, position, created_at, updated_at)
     VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
    params![id, data.name, data.description, max_pos + 1, now, now],
  )?;

  // Дефолтные статусы
  let default_statuses = [
    ("К работе", "#6366f1", 0),
    ("В работе", "#f59e0b", 1),
    ("Сделано", "#22c55e", 2),
  ];
  for (name, color, position) in &default_statuses {
    status_repo::create_internal(conn, &id, name, color, *position)?;
  }

  // Дефолтные приоритеты
  let default_priorities = [
    ("Низкий", "#22c55e", 0),
    ("Средний", "#eab308", 1),
    ("Высокий", "#f97316", 2),
    ("Срочный", "#ef4444", 3),
  ];
  for (name, color, level) in &default_priorities {
    priority_repo::create_internal(conn, &id, name, color, *level)?;
  }

  Ok(Project {
    uuid: id,
    name: data.name,
    description: data.description,
    position: max_pos + 1,
    created_at: now.clone(),
    updated_at: now,
  })
}

pub fn update(conn: &Connection, uuid: &str, data: UpdateProject) -> Result<Project, AppError> {
  // Проверяем существование
  let exists: bool = conn.query_row(
    "SELECT COUNT(*) > 0 FROM projects WHERE uuid = ?1",
    params![uuid],
    |row| row.get(0),
  )?;
  if !exists {
    return Err(AppError::NotFound(format!("Проект {uuid} не найден")));
  }

  let now = now();

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
