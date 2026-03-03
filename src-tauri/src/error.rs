use serde::Serialize;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
  #[error("Ошибка базы данных: {0}")]
  Database(#[from] rusqlite::Error),

  #[error("Не найдено: {0}")]
  NotFound(String),
}

impl Serialize for AppError {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    serializer.serialize_str(&self.to_string())
  }
}
