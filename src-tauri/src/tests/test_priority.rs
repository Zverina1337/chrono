use super::{create_test_project, setup_db};
use crate::models::priority::{CreatePriority, UpdatePriority};
use crate::repository::priority_repo;

#[test]
fn test_get_priorities_by_project() {
  let conn = setup_db();
  let project_uuid = create_test_project(&conn);

  let priorities = priority_repo::get_by_project(&conn, &project_uuid).unwrap();
  assert_eq!(priorities.len(), 4);
  // Проверяем сортировку по level
  assert_eq!(priorities[0].level, 0);
  assert_eq!(priorities[1].level, 1);
  assert_eq!(priorities[2].level, 2);
  assert_eq!(priorities[3].level, 3);
}

#[test]
fn test_get_priorities_empty_project() {
  let conn = setup_db();
  let priorities = priority_repo::get_by_project(&conn, "nonexistent").unwrap();
  assert!(priorities.is_empty());
}

#[test]
fn test_create_priority() {
  let conn = setup_db();
  let project_uuid = create_test_project(&conn);

  let priority = priority_repo::create(
    &conn,
    CreatePriority {
      project_uuid: project_uuid.clone(),
      name: "Критический".to_string(),
      color: "#dc2626".to_string(),
      level: 4,
    },
  )
  .unwrap();

  assert_eq!(priority.name, "Критический");
  assert_eq!(priority.color, "#dc2626");
  assert_eq!(priority.level, 4);
  assert_eq!(priority.project_uuid, project_uuid);
}

#[test]
fn test_update_priority_name() {
  let conn = setup_db();
  let project_uuid = create_test_project(&conn);
  let priorities = priority_repo::get_by_project(&conn, &project_uuid).unwrap();

  let updated = priority_repo::update(
    &conn,
    &priorities[0].uuid,
    UpdatePriority {
      name: Some("Минимальный".to_string()),
      color: None,
      level: None,
    },
  )
  .unwrap();

  assert_eq!(updated.name, "Минимальный");
}

#[test]
fn test_update_priority_level() {
  let conn = setup_db();
  let project_uuid = create_test_project(&conn);
  let priorities = priority_repo::get_by_project(&conn, &project_uuid).unwrap();

  let updated = priority_repo::update(
    &conn,
    &priorities[0].uuid,
    UpdatePriority {
      name: None,
      color: None,
      level: Some(99),
    },
  )
  .unwrap();

  assert_eq!(updated.level, 99);
}

#[test]
fn test_update_priority_not_found() {
  let conn = setup_db();
  let result = priority_repo::update(
    &conn,
    "nonexistent",
    UpdatePriority {
      name: Some("Тест".to_string()),
      color: None,
      level: None,
    },
  );
  assert!(result.is_err());
}

#[test]
fn test_delete_priority() {
  let conn = setup_db();
  let project_uuid = create_test_project(&conn);
  let priorities = priority_repo::get_by_project(&conn, &project_uuid).unwrap();

  priority_repo::delete(&conn, &priorities[0].uuid).unwrap();

  let after = priority_repo::get_by_project(&conn, &project_uuid).unwrap();
  assert_eq!(after.len(), 3);
}

#[test]
fn test_delete_priority_not_found() {
  let conn = setup_db();
  let result = priority_repo::delete(&conn, "nonexistent");
  assert!(result.is_err());
}

#[test]
fn test_delete_priority_sets_task_null() {
  let conn = setup_db();
  let project_uuid = create_test_project(&conn);
  let priorities = priority_repo::get_by_project(&conn, &project_uuid).unwrap();
  let priority_uuid = &priorities[0].uuid;

  let task = crate::repository::task_repo::create(
    &conn,
    crate::models::task::CreateTask {
      project_uuid: project_uuid.clone(),
      status_uuid: None,
      priority_uuid: Some(priority_uuid.clone()),
      name: "Задача".to_string(),
      description: "".to_string(),
      due_date: None,
      start_date: None,
      estimated_minutes: None,
    },
  )
  .unwrap();

  priority_repo::delete(&conn, priority_uuid).unwrap();

  let task_after = crate::repository::task_repo::get_by_uuid(&conn, &task.uuid).unwrap();
  assert!(task_after.priority_uuid.is_none());
}
