mod test_label;
mod test_migrations;
mod test_priority;
mod test_project;
mod test_status;
mod test_subtask;
mod test_task;
mod test_time_entry;

use rusqlite::Connection;

use crate::db::migrations::run_migrations;

/// Создаёт in-memory БД с миграциями и PRAGMA
pub fn setup_db() -> Connection {
  let conn = Connection::open_in_memory().unwrap();
  conn.execute_batch("PRAGMA foreign_keys=ON;").unwrap();
  run_migrations(&conn).unwrap();
  conn
}

/// Хелпер: создаёт проект и возвращает его uuid
pub fn create_test_project(conn: &Connection) -> String {
  use crate::models::project::CreateProject;
  use crate::repository::project_repo;

  let project = project_repo::create(
    conn,
    CreateProject {
      name: "Тестовый проект".to_string(),
      description: "".to_string(),
    },
  )
  .unwrap();
  project.uuid
}

/// Хелпер: создаёт задачу в проекте и возвращает (project_uuid, task_uuid)
pub fn create_test_task(conn: &Connection) -> (String, String) {
  use crate::models::task::CreateTask;
  use crate::repository::task_repo;

  let project_uuid = create_test_project(conn);

  let task = task_repo::create(
    conn,
    CreateTask {
      project_uuid: project_uuid.clone(),
      status_uuid: None,
      priority_uuid: None,
      name: "Тестовая задача".to_string(),
      description: "".to_string(),
      due_date: None,
      start_date: None,
      estimated_minutes: None,
    },
  )
  .unwrap();

  (project_uuid, task.uuid)
}
