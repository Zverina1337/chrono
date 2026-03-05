use super::{create_test_project, setup_db};
use crate::models::status::{CreateStatus, UpdateStatus};
use crate::repository::status_repo;

#[test]
fn test_get_statuses_by_project() {
  let conn = setup_db();
  let project_uuid = create_test_project(&conn);

  // Дефолтные статусы создаются при создании проекта
  let statuses = status_repo::get_by_project(&conn, &project_uuid).unwrap();
  assert_eq!(statuses.len(), 3);
  // Проверяем сортировку по position
  assert_eq!(statuses[0].position, 0);
  assert_eq!(statuses[1].position, 1);
  assert_eq!(statuses[2].position, 2);
}

#[test]
fn test_get_statuses_empty_project() {
  let conn = setup_db();
  let statuses = status_repo::get_by_project(&conn, "nonexistent").unwrap();
  assert!(statuses.is_empty());
}

#[test]
fn test_create_status() {
  let conn = setup_db();
  let project_uuid = create_test_project(&conn);

  let status = status_repo::create(
    &conn,
    CreateStatus {
      project_uuid: project_uuid.clone(),
      name: "На ревью".to_string(),
      color: "#8b5cf6".to_string(),
    },
  )
  .unwrap();

  assert_eq!(status.name, "На ревью");
  assert_eq!(status.color, "#8b5cf6");
  assert_eq!(status.project_uuid, project_uuid);
  // position = 3 (после трёх дефолтных)
  assert_eq!(status.position, 3);
}

#[test]
fn test_create_status_default_color() {
  let conn = setup_db();
  let project_uuid = create_test_project(&conn);

  let status = status_repo::create(
    &conn,
    CreateStatus {
      project_uuid: project_uuid.clone(),
      name: "Новый".to_string(),
      color: "#6366f1".to_string(),
    },
  )
  .unwrap();

  assert_eq!(status.color, "#6366f1");
}

#[test]
fn test_update_status_name() {
  let conn = setup_db();
  let project_uuid = create_test_project(&conn);

  let statuses = status_repo::get_by_project(&conn, &project_uuid).unwrap();
  let first = &statuses[0];

  let updated = status_repo::update(
    &conn,
    &first.uuid,
    UpdateStatus {
      name: Some("Переименованный".to_string()),
      color: None,
      position: None,
    },
  )
  .unwrap();

  assert_eq!(updated.name, "Переименованный");
  assert_eq!(updated.color, first.color);
}

#[test]
fn test_update_status_color() {
  let conn = setup_db();
  let project_uuid = create_test_project(&conn);
  let statuses = status_repo::get_by_project(&conn, &project_uuid).unwrap();

  let updated = status_repo::update(
    &conn,
    &statuses[0].uuid,
    UpdateStatus {
      name: None,
      color: Some("#ff0000".to_string()),
      position: None,
    },
  )
  .unwrap();

  assert_eq!(updated.color, "#ff0000");
}

#[test]
fn test_update_status_position() {
  let conn = setup_db();
  let project_uuid = create_test_project(&conn);
  let statuses = status_repo::get_by_project(&conn, &project_uuid).unwrap();

  let updated = status_repo::update(
    &conn,
    &statuses[0].uuid,
    UpdateStatus {
      name: None,
      color: None,
      position: Some(10),
    },
  )
  .unwrap();

  assert_eq!(updated.position, 10);
}

#[test]
fn test_update_status_not_found() {
  let conn = setup_db();
  let result = status_repo::update(
    &conn,
    "nonexistent",
    UpdateStatus {
      name: Some("Тест".to_string()),
      color: None,
      position: None,
    },
  );
  assert!(result.is_err());
}

#[test]
fn test_delete_status() {
  let conn = setup_db();
  let project_uuid = create_test_project(&conn);
  let statuses = status_repo::get_by_project(&conn, &project_uuid).unwrap();
  let count_before = statuses.len();

  status_repo::delete(&conn, &statuses[0].uuid).unwrap();

  let statuses_after = status_repo::get_by_project(&conn, &project_uuid).unwrap();
  assert_eq!(statuses_after.len(), count_before - 1);
}

#[test]
fn test_delete_status_not_found() {
  let conn = setup_db();
  let result = status_repo::delete(&conn, "nonexistent");
  assert!(result.is_err());
}

#[test]
fn test_delete_status_sets_task_null() {
  let conn = setup_db();
  let project_uuid = create_test_project(&conn);
  let statuses = status_repo::get_by_project(&conn, &project_uuid).unwrap();
  let status_uuid = &statuses[0].uuid;

  // Создаём задачу с этим статусом
  let task = crate::repository::task_repo::create(
    &conn,
    crate::models::task::CreateTask {
      project_uuid: project_uuid.clone(),
      status_uuid: Some(status_uuid.clone()),
      priority_uuid: None,
      name: "Задача".to_string(),
      description: "".to_string(),
      due_date: None,
      start_date: None,
      estimated_seconds: None,
    },
  )
  .unwrap();

  assert_eq!(task.status_uuid.as_deref(), Some(status_uuid.as_str()));

  // Удаляем статус — задача должна получить status_uuid = NULL
  status_repo::delete(&conn, status_uuid).unwrap();

  let task_after = crate::repository::task_repo::get_by_uuid(&conn, &task.uuid).unwrap();
  assert!(task_after.status_uuid.is_none());
}
