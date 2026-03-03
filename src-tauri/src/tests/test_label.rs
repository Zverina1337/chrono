use super::{create_test_task, setup_db};
use crate::models::label::CreateLabel;
use crate::repository::label_repo;

#[test]
fn test_create_label() {
  let conn = setup_db();

  let label = label_repo::create(
    &conn,
    CreateLabel {
      name: "Баг".to_string(),
      color: "#ef4444".to_string(),
    },
  )
  .unwrap();

  assert_eq!(label.name, "Баг");
  assert_eq!(label.color, "#ef4444");
  assert!(!label.uuid.is_empty());
}

#[test]
fn test_create_label_default_color() {
  let conn = setup_db();

  let label = label_repo::create(
    &conn,
    CreateLabel {
      name: "Фича".to_string(),
      color: "#6366f1".to_string(),
    },
  )
  .unwrap();

  assert_eq!(label.color, "#6366f1");
}

#[test]
fn test_create_label_unique_name() {
  let conn = setup_db();

  label_repo::create(
    &conn,
    CreateLabel {
      name: "Баг".to_string(),
      color: "#ef4444".to_string(),
    },
  )
  .unwrap();

  // Повторное создание с тем же именем — ошибка
  let result = label_repo::create(
    &conn,
    CreateLabel {
      name: "Баг".to_string(),
      color: "#000000".to_string(),
    },
  );
  assert!(result.is_err());
}

#[test]
fn test_get_all_labels_empty() {
  let conn = setup_db();
  let labels = label_repo::get_all(&conn).unwrap();
  assert!(labels.is_empty());
}

#[test]
fn test_get_all_labels_sorted_by_name() {
  let conn = setup_db();

  label_repo::create(
    &conn,
    CreateLabel {
      name: "Фича".to_string(),
      color: "#6366f1".to_string(),
    },
  )
  .unwrap();

  label_repo::create(
    &conn,
    CreateLabel {
      name: "Баг".to_string(),
      color: "#ef4444".to_string(),
    },
  )
  .unwrap();

  let labels = label_repo::get_all(&conn).unwrap();
  assert_eq!(labels.len(), 2);
  // Сортировка по name ASC
  assert_eq!(labels[0].name, "Баг");
  assert_eq!(labels[1].name, "Фича");
}

#[test]
fn test_delete_label() {
  let conn = setup_db();

  let label = label_repo::create(
    &conn,
    CreateLabel {
      name: "Удаляемый".to_string(),
      color: "#6366f1".to_string(),
    },
  )
  .unwrap();

  label_repo::delete(&conn, &label.uuid).unwrap();

  let labels = label_repo::get_all(&conn).unwrap();
  assert!(labels.is_empty());
}

#[test]
fn test_delete_label_not_found() {
  let conn = setup_db();
  let result = label_repo::delete(&conn, "nonexistent");
  assert!(result.is_err());
}

#[test]
fn test_add_label_to_task() {
  let conn = setup_db();
  let (_, task_uuid) = create_test_task(&conn);

  let label = label_repo::create(
    &conn,
    CreateLabel {
      name: "Баг".to_string(),
      color: "#ef4444".to_string(),
    },
  )
  .unwrap();

  label_repo::add_to_task(&conn, &task_uuid, &label.uuid).unwrap();

  // Проверяем через SQL
  let count: i32 = conn
    .query_row(
      "SELECT COUNT(*) FROM task_labels WHERE task_uuid = ?1 AND label_uuid = ?2",
      rusqlite::params![task_uuid, label.uuid],
      |row| row.get(0),
    )
    .unwrap();
  assert_eq!(count, 1);
}

#[test]
fn test_add_label_to_task_idempotent() {
  let conn = setup_db();
  let (_, task_uuid) = create_test_task(&conn);

  let label = label_repo::create(
    &conn,
    CreateLabel {
      name: "Баг".to_string(),
      color: "#ef4444".to_string(),
    },
  )
  .unwrap();

  // Добавляем дважды — не должно падать
  label_repo::add_to_task(&conn, &task_uuid, &label.uuid).unwrap();
  label_repo::add_to_task(&conn, &task_uuid, &label.uuid).unwrap();

  let count: i32 = conn
    .query_row(
      "SELECT COUNT(*) FROM task_labels WHERE task_uuid = ?1",
      rusqlite::params![task_uuid],
      |row| row.get(0),
    )
    .unwrap();
  assert_eq!(count, 1);
}

#[test]
fn test_remove_label_from_task() {
  let conn = setup_db();
  let (_, task_uuid) = create_test_task(&conn);

  let label = label_repo::create(
    &conn,
    CreateLabel {
      name: "Баг".to_string(),
      color: "#ef4444".to_string(),
    },
  )
  .unwrap();

  label_repo::add_to_task(&conn, &task_uuid, &label.uuid).unwrap();
  label_repo::remove_from_task(&conn, &task_uuid, &label.uuid).unwrap();

  let count: i32 = conn
    .query_row(
      "SELECT COUNT(*) FROM task_labels WHERE task_uuid = ?1",
      rusqlite::params![task_uuid],
      |row| row.get(0),
    )
    .unwrap();
  assert_eq!(count, 0);
}

#[test]
fn test_delete_label_cascades_task_labels() {
  let conn = setup_db();
  let (_, task_uuid) = create_test_task(&conn);

  let label = label_repo::create(
    &conn,
    CreateLabel {
      name: "Баг".to_string(),
      color: "#ef4444".to_string(),
    },
  )
  .unwrap();

  label_repo::add_to_task(&conn, &task_uuid, &label.uuid).unwrap();
  label_repo::delete(&conn, &label.uuid).unwrap();

  let count: i32 = conn
    .query_row(
      "SELECT COUNT(*) FROM task_labels WHERE task_uuid = ?1",
      rusqlite::params![task_uuid],
      |row| row.get(0),
    )
    .unwrap();
  assert_eq!(count, 0);
}

#[test]
fn test_multiple_labels_on_task() {
  let conn = setup_db();
  let (_, task_uuid) = create_test_task(&conn);

  let l1 = label_repo::create(
    &conn,
    CreateLabel {
      name: "Баг".to_string(),
      color: "#ef4444".to_string(),
    },
  )
  .unwrap();

  let l2 = label_repo::create(
    &conn,
    CreateLabel {
      name: "Срочно".to_string(),
      color: "#f97316".to_string(),
    },
  )
  .unwrap();

  label_repo::add_to_task(&conn, &task_uuid, &l1.uuid).unwrap();
  label_repo::add_to_task(&conn, &task_uuid, &l2.uuid).unwrap();

  let count: i32 = conn
    .query_row(
      "SELECT COUNT(*) FROM task_labels WHERE task_uuid = ?1",
      rusqlite::params![task_uuid],
      |row| row.get(0),
    )
    .unwrap();
  assert_eq!(count, 2);
}
