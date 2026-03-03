pub mod migrations;

use rusqlite::Connection;
use std::path::PathBuf;

use crate::error::AppError;

/// Инициализирует подключение к БД: PRAGMA + миграции
pub fn init_db(path: &PathBuf) -> Result<Connection, AppError> {
  let conn = Connection::open(path)?;

  // WAL для лучшей производительности
  conn.execute_batch("PRAGMA journal_mode=WAL;")?;
  // Включаем каскадные удаления
  conn.execute_batch("PRAGMA foreign_keys=ON;")?;

  migrations::run_migrations(&conn)?;

  Ok(conn)
}
