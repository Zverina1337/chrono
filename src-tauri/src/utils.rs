use rusqlite::{params, Connection};

use crate::error::AppError;

/// Текущее UTC-время в формате "YYYY-MM-DD HH:MM:SS"
pub fn now_utc() -> String {
  chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

/// Следующая позиция для сортируемых сущностей (MAX(position) + 1)
pub fn next_position(
  conn: &Connection,
  table: &str,
  filter_col: &str,
  filter_val: &str,
) -> Result<i32, AppError> {
  let sql = format!(
    "SELECT COALESCE(MAX(position), -1) FROM {table} WHERE {filter_col} = ?1"
  );
  let max_pos: i32 = conn.query_row(&sql, params![filter_val], |row| row.get(0))?;
  Ok(max_pos + 1)
}

/// Следующая позиция без фильтра (для projects)
pub fn next_position_global(conn: &Connection, table: &str) -> Result<i32, AppError> {
  let sql = format!("SELECT COALESCE(MAX(position), -1) FROM {table}");
  let max_pos: i32 = conn.query_row(&sql, [], |row| row.get(0))?;
  Ok(max_pos + 1)
}

/// Конвертация SQLite INTEGER (0/1) в bool
pub fn bool_from_i32(v: i32) -> bool {
  v != 0
}
