use super::setup_db;
use rusqlite::params;

#[test]
fn test_tables_created() {
  let conn = setup_db();

  // Проверяем, что все 8 таблиц существуют
  let tables = [
    "projects",
    "statuses",
    "priorities",
    "tasks",
    "subtasks",
    "labels",
    "task_labels",
    "time_entries",
  ];

  for table in &tables {
    let count: i32 = conn
      .query_row(
        &format!("SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='{table}'"),
        [],
        |row| row.get(0),
      )
      .unwrap();
    assert_eq!(count, 1, "Таблица {table} не найдена");
  }
}

#[test]
fn test_user_version_set() {
  let conn = setup_db();
  let version: i32 = conn
    .pragma_query_value(None, "user_version", |row| row.get(0))
    .unwrap();
  assert_eq!(version, 1);
}

#[test]
fn test_foreign_keys_enabled() {
  let conn = setup_db();
  let fk: i32 = conn
    .pragma_query_value(None, "foreign_keys", |row| row.get(0))
    .unwrap();
  assert_eq!(fk, 1);
}

#[test]
fn test_migrations_idempotent() {
  let conn = setup_db();
  // Повторный вызов не должен падать
  crate::db::migrations::run_migrations(&conn).unwrap();
  let version: i32 = conn
    .pragma_query_value(None, "user_version", |row| row.get(0))
    .unwrap();
  assert_eq!(version, 1);
}

#[test]
fn test_cascade_delete_project() {
  let conn = setup_db();

  // Создаём проект (с дефолтными статусами/приоритетами)
  let project_uuid = super::create_test_project(&conn);

  // Проверяем наличие статусов и приоритетов
  let status_count: i32 = conn
    .query_row(
      "SELECT COUNT(*) FROM statuses WHERE project_uuid = ?1",
      params![project_uuid],
      |row| row.get(0),
    )
    .unwrap();
  assert_eq!(status_count, 3);

  let priority_count: i32 = conn
    .query_row(
      "SELECT COUNT(*) FROM priorities WHERE project_uuid = ?1",
      params![project_uuid],
      |row| row.get(0),
    )
    .unwrap();
  assert_eq!(priority_count, 4);

  // Удаляем проект
  conn
    .execute(
      "DELETE FROM projects WHERE uuid = ?1",
      params![project_uuid],
    )
    .unwrap();

  // Проверяем каскадное удаление
  let status_count: i32 = conn
    .query_row(
      "SELECT COUNT(*) FROM statuses WHERE project_uuid = ?1",
      params![project_uuid],
      |row| row.get(0),
    )
    .unwrap();
  assert_eq!(status_count, 0);

  let priority_count: i32 = conn
    .query_row(
      "SELECT COUNT(*) FROM priorities WHERE project_uuid = ?1",
      params![project_uuid],
      |row| row.get(0),
    )
    .unwrap();
  assert_eq!(priority_count, 0);
}
