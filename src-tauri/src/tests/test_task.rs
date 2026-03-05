use super::{create_test_project, setup_db};
use crate::models::task::{CreateTask, UpdateTask};
use crate::repository::{status_repo, task_repo};

fn make_task(project_uuid: &str) -> CreateTask {
  CreateTask {
    project_uuid: project_uuid.to_string(),
    status_uuid: None,
    priority_uuid: None,
    name: "Задача".to_string(),
    description: "".to_string(),
    due_date: None,
    start_date: None,
    estimated_seconds: None,
  }
}

#[test]
fn test_create_task() {
  let conn = setup_db();
  let project_uuid = create_test_project(&conn);

  let task = task_repo::create(&conn, make_task(&project_uuid)).unwrap();

  assert_eq!(task.name, "Задача");
  assert_eq!(task.project_uuid, project_uuid);
  assert_eq!(task.position, 0);
  assert!(task.status_uuid.is_none());
  assert!(task.priority_uuid.is_none());
  assert_eq!(task.description, "");
}

#[test]
fn test_create_task_with_status_and_priority() {
  let conn = setup_db();
  let project_uuid = create_test_project(&conn);

  let statuses = status_repo::get_by_project(&conn, &project_uuid).unwrap();
  let priorities =
    crate::repository::priority_repo::get_by_project(&conn, &project_uuid).unwrap();

  let task = task_repo::create(
    &conn,
    CreateTask {
      project_uuid: project_uuid.clone(),
      status_uuid: Some(statuses[0].uuid.clone()),
      priority_uuid: Some(priorities[2].uuid.clone()),
      name: "Полная задача".to_string(),
      description: "Подробное описание".to_string(),
      due_date: Some("2026-12-31".to_string()),
      start_date: Some("2026-03-01".to_string()),
      estimated_seconds: Some(120),
    },
  )
  .unwrap();

  assert_eq!(task.status_uuid.as_deref(), Some(statuses[0].uuid.as_str()));
  assert_eq!(
    task.priority_uuid.as_deref(),
    Some(priorities[2].uuid.as_str())
  );
  assert_eq!(task.description, "Подробное описание");
  assert_eq!(task.due_date.as_deref(), Some("2026-12-31"));
  assert_eq!(task.start_date.as_deref(), Some("2026-03-01"));
  assert_eq!(task.estimated_seconds, Some(120));
}

#[test]
fn test_create_task_auto_position() {
  let conn = setup_db();
  let project_uuid = create_test_project(&conn);

  let t1 = task_repo::create(&conn, make_task(&project_uuid)).unwrap();
  let t2 = task_repo::create(&conn, make_task(&project_uuid)).unwrap();
  let t3 = task_repo::create(&conn, make_task(&project_uuid)).unwrap();

  assert_eq!(t1.position, 0);
  assert_eq!(t2.position, 1);
  assert_eq!(t3.position, 2);
}

#[test]
fn test_get_tasks_by_project() {
  let conn = setup_db();
  let project_uuid = create_test_project(&conn);

  task_repo::create(&conn, make_task(&project_uuid)).unwrap();
  task_repo::create(&conn, make_task(&project_uuid)).unwrap();

  let tasks = task_repo::get_by_project(&conn, &project_uuid).unwrap();
  assert_eq!(tasks.len(), 2);
  assert!(tasks[0].position <= tasks[1].position);
}

#[test]
fn test_get_tasks_empty_project() {
  let conn = setup_db();
  let tasks = task_repo::get_by_project(&conn, "nonexistent").unwrap();
  assert!(tasks.is_empty());
}

#[test]
fn test_get_task_by_uuid() {
  let conn = setup_db();
  let project_uuid = create_test_project(&conn);
  let task = task_repo::create(&conn, make_task(&project_uuid)).unwrap();

  let found = task_repo::get_by_uuid(&conn, &task.uuid).unwrap();
  assert_eq!(found.uuid, task.uuid);
  assert_eq!(found.name, task.name);
}

#[test]
fn test_get_task_not_found() {
  let conn = setup_db();
  let result = task_repo::get_by_uuid(&conn, "nonexistent");
  assert!(result.is_err());
}

#[test]
fn test_update_task_name() {
  let conn = setup_db();
  let project_uuid = create_test_project(&conn);
  let task = task_repo::create(&conn, make_task(&project_uuid)).unwrap();

  let updated = task_repo::update(
    &conn,
    &task.uuid,
    UpdateTask {
      name: Some("Новое имя".to_string()),
      status_uuid: None,
      priority_uuid: None,
      description: None,
      due_date: None,
      start_date: None,
      estimated_seconds: None,
      position: None,
    },
  )
  .unwrap();

  assert_eq!(updated.name, "Новое имя");
}

#[test]
fn test_update_task_status() {
  let conn = setup_db();
  let project_uuid = create_test_project(&conn);
  let task = task_repo::create(&conn, make_task(&project_uuid)).unwrap();
  let statuses = status_repo::get_by_project(&conn, &project_uuid).unwrap();

  // Устанавливаем статус
  let updated = task_repo::update(
    &conn,
    &task.uuid,
    UpdateTask {
      status_uuid: Some(Some(statuses[1].uuid.clone())),
      priority_uuid: None,
      name: None,
      description: None,
      due_date: None,
      start_date: None,
      estimated_seconds: None,
      position: None,
    },
  )
  .unwrap();

  assert_eq!(
    updated.status_uuid.as_deref(),
    Some(statuses[1].uuid.as_str())
  );

  // Сбрасываем статус в NULL
  let cleared = task_repo::update(
    &conn,
    &task.uuid,
    UpdateTask {
      status_uuid: Some(None),
      priority_uuid: None,
      name: None,
      description: None,
      due_date: None,
      start_date: None,
      estimated_seconds: None,
      position: None,
    },
  )
  .unwrap();

  assert!(cleared.status_uuid.is_none());
}

#[test]
fn test_update_task_dates() {
  let conn = setup_db();
  let project_uuid = create_test_project(&conn);
  let task = task_repo::create(&conn, make_task(&project_uuid)).unwrap();

  let updated = task_repo::update(
    &conn,
    &task.uuid,
    UpdateTask {
      due_date: Some(Some("2026-06-15".to_string())),
      start_date: Some(Some("2026-03-01".to_string())),
      estimated_seconds: Some(Some(60)),
      status_uuid: None,
      priority_uuid: None,
      name: None,
      description: None,
      position: None,
    },
  )
  .unwrap();

  assert_eq!(updated.due_date.as_deref(), Some("2026-06-15"));
  assert_eq!(updated.start_date.as_deref(), Some("2026-03-01"));
  assert_eq!(updated.estimated_seconds, Some(60));
}

#[test]
fn test_update_task_not_found() {
  let conn = setup_db();
  let result = task_repo::update(
    &conn,
    "nonexistent",
    UpdateTask {
      name: Some("Тест".to_string()),
      status_uuid: None,
      priority_uuid: None,
      description: None,
      due_date: None,
      start_date: None,
      estimated_seconds: None,
      position: None,
    },
  );
  assert!(result.is_err());
}

#[test]
fn test_delete_task() {
  let conn = setup_db();
  let project_uuid = create_test_project(&conn);
  let task = task_repo::create(&conn, make_task(&project_uuid)).unwrap();

  task_repo::delete(&conn, &task.uuid).unwrap();

  let tasks = task_repo::get_by_project(&conn, &project_uuid).unwrap();
  assert!(tasks.is_empty());
}

#[test]
fn test_delete_task_not_found() {
  let conn = setup_db();
  let result = task_repo::delete(&conn, "nonexistent");
  assert!(result.is_err());
}

#[test]
fn test_delete_task_cascades_subtasks() {
  let conn = setup_db();
  let (_, task_uuid) = super::create_test_task(&conn);

  // Создаём подзадачу
  crate::repository::subtask_repo::create(
    &conn,
    crate::models::subtask::CreateSubtask {
      task_uuid: task_uuid.clone(),
      name: "Подзадача".to_string(),
    },
  )
  .unwrap();

  task_repo::delete(&conn, &task_uuid).unwrap();

  let subtasks = crate::repository::subtask_repo::get_by_task(&conn, &task_uuid).unwrap();
  assert!(subtasks.is_empty());
}
