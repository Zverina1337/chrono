use super::{create_test_task, setup_db};
use crate::models::subtask::{CreateSubtask, UpdateSubtask};
use crate::repository::subtask_repo;

#[test]
fn test_create_subtask() {
  let conn = setup_db();
  let (_, task_uuid) = create_test_task(&conn);

  let subtask = subtask_repo::create(
    &conn,
    CreateSubtask {
      task_uuid: task_uuid.clone(),
      name: "Подзадача 1".to_string(),
    },
  )
  .unwrap();

  assert_eq!(subtask.name, "Подзадача 1");
  assert_eq!(subtask.task_uuid, task_uuid);
  assert!(!subtask.is_completed);
  assert_eq!(subtask.position, 0);
}

#[test]
fn test_create_subtask_auto_position() {
  let conn = setup_db();
  let (_, task_uuid) = create_test_task(&conn);

  let s1 = subtask_repo::create(
    &conn,
    CreateSubtask {
      task_uuid: task_uuid.clone(),
      name: "Первая".to_string(),
    },
  )
  .unwrap();

  let s2 = subtask_repo::create(
    &conn,
    CreateSubtask {
      task_uuid: task_uuid.clone(),
      name: "Вторая".to_string(),
    },
  )
  .unwrap();

  assert_eq!(s1.position, 0);
  assert_eq!(s2.position, 1);
}

#[test]
fn test_get_subtasks_by_task() {
  let conn = setup_db();
  let (_, task_uuid) = create_test_task(&conn);

  subtask_repo::create(
    &conn,
    CreateSubtask {
      task_uuid: task_uuid.clone(),
      name: "Первая".to_string(),
    },
  )
  .unwrap();

  subtask_repo::create(
    &conn,
    CreateSubtask {
      task_uuid: task_uuid.clone(),
      name: "Вторая".to_string(),
    },
  )
  .unwrap();

  let subtasks = subtask_repo::get_by_task(&conn, &task_uuid).unwrap();
  assert_eq!(subtasks.len(), 2);
  assert_eq!(subtasks[0].name, "Первая");
  assert_eq!(subtasks[1].name, "Вторая");
}

#[test]
fn test_get_subtasks_empty() {
  let conn = setup_db();
  let subtasks = subtask_repo::get_by_task(&conn, "nonexistent").unwrap();
  assert!(subtasks.is_empty());
}

#[test]
fn test_update_subtask_toggle_completed() {
  let conn = setup_db();
  let (_, task_uuid) = create_test_task(&conn);

  let subtask = subtask_repo::create(
    &conn,
    CreateSubtask {
      task_uuid,
      name: "Тест".to_string(),
    },
  )
  .unwrap();
  assert!(!subtask.is_completed);

  // Включаем
  let updated = subtask_repo::update(
    &conn,
    &subtask.uuid,
    UpdateSubtask {
      name: None,
      is_completed: Some(true),
      position: None,
    },
  )
  .unwrap();
  assert!(updated.is_completed);

  // Выключаем
  let toggled = subtask_repo::update(
    &conn,
    &subtask.uuid,
    UpdateSubtask {
      name: None,
      is_completed: Some(false),
      position: None,
    },
  )
  .unwrap();
  assert!(!toggled.is_completed);
}

#[test]
fn test_update_subtask_name() {
  let conn = setup_db();
  let (_, task_uuid) = create_test_task(&conn);

  let subtask = subtask_repo::create(
    &conn,
    CreateSubtask {
      task_uuid,
      name: "Старое".to_string(),
    },
  )
  .unwrap();

  let updated = subtask_repo::update(
    &conn,
    &subtask.uuid,
    UpdateSubtask {
      name: Some("Новое".to_string()),
      is_completed: None,
      position: None,
    },
  )
  .unwrap();

  assert_eq!(updated.name, "Новое");
}

#[test]
fn test_update_subtask_position() {
  let conn = setup_db();
  let (_, task_uuid) = create_test_task(&conn);

  let subtask = subtask_repo::create(
    &conn,
    CreateSubtask {
      task_uuid,
      name: "Тест".to_string(),
    },
  )
  .unwrap();

  let updated = subtask_repo::update(
    &conn,
    &subtask.uuid,
    UpdateSubtask {
      name: None,
      is_completed: None,
      position: Some(5),
    },
  )
  .unwrap();

  assert_eq!(updated.position, 5);
}

#[test]
fn test_update_subtask_not_found() {
  let conn = setup_db();
  let result = subtask_repo::update(
    &conn,
    "nonexistent",
    UpdateSubtask {
      name: Some("Тест".to_string()),
      is_completed: None,
      position: None,
    },
  );
  assert!(result.is_err());
}

#[test]
fn test_delete_subtask() {
  let conn = setup_db();
  let (_, task_uuid) = create_test_task(&conn);

  let subtask = subtask_repo::create(
    &conn,
    CreateSubtask {
      task_uuid: task_uuid.clone(),
      name: "Удаляемая".to_string(),
    },
  )
  .unwrap();

  subtask_repo::delete(&conn, &subtask.uuid).unwrap();

  let subtasks = subtask_repo::get_by_task(&conn, &task_uuid).unwrap();
  assert!(subtasks.is_empty());
}

#[test]
fn test_delete_subtask_not_found() {
  let conn = setup_db();
  let result = subtask_repo::delete(&conn, "nonexistent");
  assert!(result.is_err());
}
