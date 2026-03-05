use rusqlite::Connection;

use crate::error::AppError;

/// Все SQL-миграции по порядку
const MIGRATIONS: &[&str] = &[
  // v1 — начальная схема
  "
  CREATE TABLE IF NOT EXISTS projects (
    uuid        TEXT PRIMARY KEY,
    name        TEXT NOT NULL,
    description TEXT NOT NULL DEFAULT '',
    position    INTEGER NOT NULL DEFAULT 0,
    created_at  TEXT NOT NULL,
    updated_at  TEXT NOT NULL
  );

  CREATE TABLE IF NOT EXISTS statuses (
    uuid         TEXT PRIMARY KEY,
    project_uuid TEXT NOT NULL REFERENCES projects(uuid) ON DELETE CASCADE,
    name         TEXT NOT NULL,
    color        TEXT NOT NULL DEFAULT '#6366f1',
    position     INTEGER NOT NULL DEFAULT 0,
    created_at   TEXT NOT NULL,
    updated_at   TEXT NOT NULL
  );

  CREATE TABLE IF NOT EXISTS priorities (
    uuid         TEXT PRIMARY KEY,
    project_uuid TEXT NOT NULL REFERENCES projects(uuid) ON DELETE CASCADE,
    name         TEXT NOT NULL,
    color        TEXT NOT NULL DEFAULT '#6366f1',
    level        INTEGER NOT NULL DEFAULT 0,
    created_at   TEXT NOT NULL,
    updated_at   TEXT NOT NULL
  );

  CREATE TABLE IF NOT EXISTS tasks (
    uuid              TEXT PRIMARY KEY,
    project_uuid      TEXT NOT NULL REFERENCES projects(uuid) ON DELETE CASCADE,
    status_uuid       TEXT REFERENCES statuses(uuid) ON DELETE SET NULL,
    priority_uuid     TEXT REFERENCES priorities(uuid) ON DELETE SET NULL,
    name              TEXT NOT NULL,
    description       TEXT NOT NULL DEFAULT '',
    due_date          TEXT,
    start_date        TEXT,
    estimated_minutes INTEGER,
    position          INTEGER NOT NULL DEFAULT 0,
    created_at        TEXT NOT NULL,
    updated_at        TEXT NOT NULL
  );

  CREATE TABLE IF NOT EXISTS subtasks (
    uuid         TEXT PRIMARY KEY,
    task_uuid    TEXT NOT NULL REFERENCES tasks(uuid) ON DELETE CASCADE,
    name         TEXT NOT NULL,
    is_completed INTEGER NOT NULL DEFAULT 0,
    position     INTEGER NOT NULL DEFAULT 0,
    created_at   TEXT NOT NULL,
    updated_at   TEXT NOT NULL
  );

  CREATE TABLE IF NOT EXISTS labels (
    uuid  TEXT PRIMARY KEY,
    name  TEXT NOT NULL UNIQUE,
    color TEXT NOT NULL DEFAULT '#6366f1'
  );

  CREATE TABLE IF NOT EXISTS task_labels (
    task_uuid  TEXT NOT NULL REFERENCES tasks(uuid) ON DELETE CASCADE,
    label_uuid TEXT NOT NULL REFERENCES labels(uuid) ON DELETE CASCADE,
    PRIMARY KEY (task_uuid, label_uuid)
  );

  CREATE TABLE IF NOT EXISTS time_entries (
    uuid             TEXT PRIMARY KEY,
    task_uuid        TEXT NOT NULL REFERENCES tasks(uuid) ON DELETE CASCADE,
    started_at       TEXT NOT NULL,
    ended_at         TEXT,
    duration_minutes INTEGER,
    description      TEXT NOT NULL DEFAULT '',
    created_at       TEXT NOT NULL
  );
  ",
  // v2 — переименование колонок + индексы на FK
  "
  ALTER TABLE tasks RENAME COLUMN estimated_minutes TO estimated_seconds;
  ALTER TABLE time_entries RENAME COLUMN duration_minutes TO duration_seconds;

  CREATE INDEX IF NOT EXISTS idx_statuses_project ON statuses(project_uuid);
  CREATE INDEX IF NOT EXISTS idx_priorities_project ON priorities(project_uuid);
  CREATE INDEX IF NOT EXISTS idx_tasks_project ON tasks(project_uuid);
  CREATE INDEX IF NOT EXISTS idx_tasks_status ON tasks(status_uuid);
  CREATE INDEX IF NOT EXISTS idx_tasks_priority ON tasks(priority_uuid);
  CREATE INDEX IF NOT EXISTS idx_subtasks_task ON subtasks(task_uuid);
  CREATE INDEX IF NOT EXISTS idx_time_entries_task ON time_entries(task_uuid);
  CREATE INDEX IF NOT EXISTS idx_task_labels_task ON task_labels(task_uuid);
  CREATE INDEX IF NOT EXISTS idx_task_labels_label ON task_labels(label_uuid);
  ",
];

/// Выполняет миграции, отслеживая версию через user_version
pub fn run_migrations(conn: &Connection) -> Result<(), AppError> {
  let current_version: i32 = conn.pragma_query_value(None, "user_version", |row| row.get(0))?;

  for (i, migration) in MIGRATIONS.iter().enumerate() {
    let version = (i + 1) as i32;
    if version > current_version {
      conn.execute_batch(migration)?;
      conn.pragma_update(None, "user_version", version)?;
    }
  }

  Ok(())
}
