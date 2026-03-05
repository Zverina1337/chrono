mod commands;
mod db;
mod error;
mod models;
mod repository;
mod utils;

#[cfg(test)]
mod tests;

use std::fs;
use std::sync::Mutex;

use commands::label_cmd::*;
use commands::priority_cmd::*;
use commands::project_cmd::*;
use commands::status_cmd::*;
use commands::subtask_cmd::*;
use commands::task_cmd::*;
use commands::time_entry_cmd::*;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_log::Builder::new().build())
    .plugin(tauri_plugin_opener::init())
    .setup(|app| {
      use tauri::{Manager, WebviewUrl, WebviewWindowBuilder};

      // Создаём окно программно, чтобы отключить нативный file drop handler
      // (он перехватывает drag-события до WebView и ломает HTML5 Drag and Drop API)
      WebviewWindowBuilder::new(app, "main", WebviewUrl::App("index.html".into()))
        .title("tauri-app")
        .inner_size(800.0, 600.0)
        .disable_drag_drop_handler()
        .build()?;

      let app_dir = app.path().app_data_dir()?;
      fs::create_dir_all(&app_dir)?;
      let db_path = app_dir.join("chrono.db");

      let conn = db::init_db(&db_path).expect("Не удалось инициализировать БД");
      app.manage(Mutex::new(conn));

      Ok(())
    })
    .invoke_handler(tauri::generate_handler![
      // Projects (4)
      get_all_projects,
      create_project,
      update_project,
      delete_project,
      // Statuses (4)
      get_statuses_by_project,
      create_status,
      update_status,
      delete_status,
      // Priorities (4)
      get_priorities_by_project,
      create_priority,
      update_priority,
      delete_priority,
      // Tasks (5)
      get_tasks_by_project,
      get_task,
      create_task,
      update_task,
      delete_task,
      // Subtasks (4)
      get_subtasks_by_task,
      create_subtask,
      update_subtask,
      delete_subtask,
      // Labels (5)
      get_all_labels,
      create_label,
      delete_label,
      add_label_to_task,
      remove_label_from_task,
      // Time entries (4)
      get_time_entries_by_task,
      create_time_entry,
      update_time_entry,
      delete_time_entry,
    ])
    .run(tauri::generate_context!())
    .expect("Ошибка при запуске приложения");
}
