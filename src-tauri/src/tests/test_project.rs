use super::setup_db;
use crate::models::project::{CreateProject, UpdateProject};
use crate::repository::project_repo;

#[test]
fn test_create_project() {
  let conn = setup_db();
  let project = project_repo::create(
    &conn,
    CreateProject {
      name: "Тест".to_string(),
      description: "Описание".to_string(),
    },
  )
  .unwrap();

  assert_eq!(project.name, "Тест");
  assert_eq!(project.description, "Описание");
  assert_eq!(project.position, 0);
  assert!(!project.uuid.is_empty());
  assert!(!project.created_at.is_empty());
}

#[test]
fn test_create_project_default_description() {
  let conn = setup_db();
  let project = project_repo::create(
    &conn,
    CreateProject {
      name: "Тест".to_string(),
      description: "".to_string(),
    },
  )
  .unwrap();

  assert_eq!(project.description, "");
}

#[test]
fn test_create_project_auto_position() {
  let conn = setup_db();

  let p1 = project_repo::create(
    &conn,
    CreateProject {
      name: "Первый".to_string(),
      description: "".to_string(),
    },
  )
  .unwrap();

  let p2 = project_repo::create(
    &conn,
    CreateProject {
      name: "Второй".to_string(),
      description: "".to_string(),
    },
  )
  .unwrap();

  assert_eq!(p1.position, 0);
  assert_eq!(p2.position, 1);
}

#[test]
fn test_create_project_creates_default_statuses() {
  let conn = setup_db();
  let project = project_repo::create(
    &conn,
    CreateProject {
      name: "Тест".to_string(),
      description: "".to_string(),
    },
  )
  .unwrap();

  let statuses = crate::repository::status_repo::get_by_project(&conn, &project.uuid).unwrap();
  assert_eq!(statuses.len(), 3);
  assert_eq!(statuses[0].name, "К работе");
  assert_eq!(statuses[1].name, "В работе");
  assert_eq!(statuses[2].name, "Сделано");
}

#[test]
fn test_create_project_creates_default_priorities() {
  let conn = setup_db();
  let project = project_repo::create(
    &conn,
    CreateProject {
      name: "Тест".to_string(),
      description: "".to_string(),
    },
  )
  .unwrap();

  let priorities =
    crate::repository::priority_repo::get_by_project(&conn, &project.uuid).unwrap();
  assert_eq!(priorities.len(), 4);
  assert_eq!(priorities[0].name, "Низкий");
  assert_eq!(priorities[0].level, 0);
  assert_eq!(priorities[3].name, "Срочный");
  assert_eq!(priorities[3].level, 3);
}

#[test]
fn test_get_all_projects_empty() {
  let conn = setup_db();
  let projects = project_repo::get_all(&conn).unwrap();
  assert!(projects.is_empty());
}

#[test]
fn test_get_all_projects_sorted() {
  let conn = setup_db();

  project_repo::create(
    &conn,
    CreateProject {
      name: "Второй".to_string(),
      description: "".to_string(),
    },
  )
  .unwrap();

  project_repo::create(
    &conn,
    CreateProject {
      name: "Первый".to_string(),
      description: "".to_string(),
    },
  )
  .unwrap();

  let projects = project_repo::get_all(&conn).unwrap();
  assert_eq!(projects.len(), 2);
  assert_eq!(projects[0].name, "Второй");
  assert_eq!(projects[1].name, "Первый");
}

#[test]
fn test_update_project_name() {
  let conn = setup_db();
  let project = project_repo::create(
    &conn,
    CreateProject {
      name: "Старое".to_string(),
      description: "".to_string(),
    },
  )
  .unwrap();

  let updated = project_repo::update(
    &conn,
    &project.uuid,
    UpdateProject {
      name: Some("Новое".to_string()),
      description: None,
      position: None,
    },
  )
  .unwrap();

  assert_eq!(updated.name, "Новое");
  assert_eq!(updated.description, "");
}

#[test]
fn test_update_project_partial() {
  let conn = setup_db();
  let project = project_repo::create(
    &conn,
    CreateProject {
      name: "Имя".to_string(),
      description: "Описание".to_string(),
    },
  )
  .unwrap();

  // Обновляем только описание
  let updated = project_repo::update(
    &conn,
    &project.uuid,
    UpdateProject {
      name: None,
      description: Some("Новое описание".to_string()),
      position: None,
    },
  )
  .unwrap();

  assert_eq!(updated.name, "Имя");
  assert_eq!(updated.description, "Новое описание");
}

#[test]
fn test_update_project_not_found() {
  let conn = setup_db();
  let result = project_repo::update(
    &conn,
    "nonexistent-uuid",
    UpdateProject {
      name: Some("Тест".to_string()),
      description: None,
      position: None,
    },
  );

  assert!(result.is_err());
}

#[test]
fn test_delete_project() {
  let conn = setup_db();
  let project = project_repo::create(
    &conn,
    CreateProject {
      name: "Удаляемый".to_string(),
      description: "".to_string(),
    },
  )
  .unwrap();

  project_repo::delete(&conn, &project.uuid).unwrap();

  let projects = project_repo::get_all(&conn).unwrap();
  assert!(projects.is_empty());
}

#[test]
fn test_delete_project_not_found() {
  let conn = setup_db();
  let result = project_repo::delete(&conn, "nonexistent-uuid");
  assert!(result.is_err());
}

#[test]
fn test_get_by_uuid() {
  let conn = setup_db();
  let project = project_repo::create(
    &conn,
    CreateProject {
      name: "Тест".to_string(),
      description: "Описание".to_string(),
    },
  )
  .unwrap();

  let found = project_repo::get_by_uuid(&conn, &project.uuid).unwrap();
  assert_eq!(found.name, "Тест");
  assert_eq!(found.uuid, project.uuid);
}

#[test]
fn test_get_by_uuid_not_found() {
  let conn = setup_db();
  let result = project_repo::get_by_uuid(&conn, "nonexistent-uuid");
  assert!(result.is_err());
}
