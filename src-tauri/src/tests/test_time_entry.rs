use super::{create_test_task, setup_db};
use crate::models::time_entry::{CreateTimeEntry, UpdateTimeEntry};
use crate::repository::time_entry_repo;

#[test]
fn test_create_time_entry() {
  let conn = setup_db();
  let (_, task_uuid) = create_test_task(&conn);

  let entry = time_entry_repo::create(
    &conn,
    CreateTimeEntry {
      task_uuid: task_uuid.clone(),
      started_at: "2026-03-02 10:00:00".to_string(),
      ended_at: Some("2026-03-02 11:30:00".to_string()),
      duration_seconds: Some(90),
      description: "Работа над задачей".to_string(),
    },
  )
  .unwrap();

  assert_eq!(entry.task_uuid, task_uuid);
  assert_eq!(entry.started_at, "2026-03-02 10:00:00");
  assert_eq!(entry.ended_at.as_deref(), Some("2026-03-02 11:30:00"));
  assert_eq!(entry.duration_seconds, Some(90));
  assert_eq!(entry.description, "Работа над задачей");
  assert!(!entry.uuid.is_empty());
  assert!(!entry.created_at.is_empty());
}

#[test]
fn test_create_time_entry_running_timer() {
  let conn = setup_db();
  let (_, task_uuid) = create_test_task(&conn);

  // ended_at = None означает, что таймер запущен
  let entry = time_entry_repo::create(
    &conn,
    CreateTimeEntry {
      task_uuid,
      started_at: "2026-03-02 10:00:00".to_string(),
      ended_at: None,
      duration_seconds: None,
      description: "".to_string(),
    },
  )
  .unwrap();

  assert!(entry.ended_at.is_none());
  assert!(entry.duration_seconds.is_none());
}

#[test]
fn test_get_time_entries_by_task() {
  let conn = setup_db();
  let (_, task_uuid) = create_test_task(&conn);

  time_entry_repo::create(
    &conn,
    CreateTimeEntry {
      task_uuid: task_uuid.clone(),
      started_at: "2026-03-02 08:00:00".to_string(),
      ended_at: Some("2026-03-02 09:00:00".to_string()),
      duration_seconds: Some(60),
      description: "".to_string(),
    },
  )
  .unwrap();

  time_entry_repo::create(
    &conn,
    CreateTimeEntry {
      task_uuid: task_uuid.clone(),
      started_at: "2026-03-02 10:00:00".to_string(),
      ended_at: Some("2026-03-02 11:00:00".to_string()),
      duration_seconds: Some(60),
      description: "".to_string(),
    },
  )
  .unwrap();

  let entries = time_entry_repo::get_by_task(&conn, &task_uuid).unwrap();
  assert_eq!(entries.len(), 2);
  // Сортировка по started_at DESC — новейшая первая
  assert_eq!(entries[0].started_at, "2026-03-02 10:00:00");
  assert_eq!(entries[1].started_at, "2026-03-02 08:00:00");
}

#[test]
fn test_get_time_entries_empty() {
  let conn = setup_db();
  let entries = time_entry_repo::get_by_task(&conn, "nonexistent").unwrap();
  assert!(entries.is_empty());
}

#[test]
fn test_update_time_entry_stop_timer() {
  let conn = setup_db();
  let (_, task_uuid) = create_test_task(&conn);

  let entry = time_entry_repo::create(
    &conn,
    CreateTimeEntry {
      task_uuid,
      started_at: "2026-03-02 10:00:00".to_string(),
      ended_at: None,
      duration_seconds: None,
      description: "".to_string(),
    },
  )
  .unwrap();
  assert!(entry.ended_at.is_none());

  // Останавливаем таймер
  let updated = time_entry_repo::update(
    &conn,
    &entry.uuid,
    UpdateTimeEntry {
      ended_at: Some(Some("2026-03-02 11:30:00".to_string())),
      duration_seconds: Some(Some(90)),
      description: None,
    },
  )
  .unwrap();

  assert_eq!(updated.ended_at.as_deref(), Some("2026-03-02 11:30:00"));
  assert_eq!(updated.duration_seconds, Some(90));
}

#[test]
fn test_update_time_entry_description() {
  let conn = setup_db();
  let (_, task_uuid) = create_test_task(&conn);

  let entry = time_entry_repo::create(
    &conn,
    CreateTimeEntry {
      task_uuid,
      started_at: "2026-03-02 10:00:00".to_string(),
      ended_at: None,
      duration_seconds: None,
      description: "".to_string(),
    },
  )
  .unwrap();

  let updated = time_entry_repo::update(
    &conn,
    &entry.uuid,
    UpdateTimeEntry {
      ended_at: None,
      duration_seconds: None,
      description: Some("Обновлённое описание".to_string()),
    },
  )
  .unwrap();

  assert_eq!(updated.description, "Обновлённое описание");
}

#[test]
fn test_update_time_entry_clear_duration() {
  let conn = setup_db();
  let (_, task_uuid) = create_test_task(&conn);

  let entry = time_entry_repo::create(
    &conn,
    CreateTimeEntry {
      task_uuid,
      started_at: "2026-03-02 10:00:00".to_string(),
      ended_at: Some("2026-03-02 11:00:00".to_string()),
      duration_seconds: Some(60),
      description: "".to_string(),
    },
  )
  .unwrap();

  // Сбрасываем duration в NULL
  let updated = time_entry_repo::update(
    &conn,
    &entry.uuid,
    UpdateTimeEntry {
      ended_at: None,
      duration_seconds: Some(None),
      description: None,
    },
  )
  .unwrap();

  assert!(updated.duration_seconds.is_none());
}

#[test]
fn test_update_time_entry_not_found() {
  let conn = setup_db();
  let result = time_entry_repo::update(
    &conn,
    "nonexistent",
    UpdateTimeEntry {
      ended_at: None,
      duration_seconds: None,
      description: Some("Тест".to_string()),
    },
  );
  assert!(result.is_err());
}

#[test]
fn test_delete_time_entry() {
  let conn = setup_db();
  let (_, task_uuid) = create_test_task(&conn);

  let entry = time_entry_repo::create(
    &conn,
    CreateTimeEntry {
      task_uuid: task_uuid.clone(),
      started_at: "2026-03-02 10:00:00".to_string(),
      ended_at: None,
      duration_seconds: None,
      description: "".to_string(),
    },
  )
  .unwrap();

  time_entry_repo::delete(&conn, &entry.uuid).unwrap();

  let entries = time_entry_repo::get_by_task(&conn, &task_uuid).unwrap();
  assert!(entries.is_empty());
}

#[test]
fn test_delete_time_entry_not_found() {
  let conn = setup_db();
  let result = time_entry_repo::delete(&conn, "nonexistent");
  assert!(result.is_err());
}

#[test]
fn test_delete_task_cascades_time_entries() {
  let conn = setup_db();
  let (_, task_uuid) = create_test_task(&conn);

  time_entry_repo::create(
    &conn,
    CreateTimeEntry {
      task_uuid: task_uuid.clone(),
      started_at: "2026-03-02 10:00:00".to_string(),
      ended_at: None,
      duration_seconds: None,
      description: "".to_string(),
    },
  )
  .unwrap();

  crate::repository::task_repo::delete(&conn, &task_uuid).unwrap();

  let entries = time_entry_repo::get_by_task(&conn, &task_uuid).unwrap();
  assert!(entries.is_empty());
}
