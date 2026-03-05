use serde::Serialize;

#[derive(Debug, thiserror::Error)]
#[allow(dead_code)]
pub enum AppError {
  #[error("Ошибка базы данных: {0}")]
  Database(#[from] rusqlite::Error),

  #[error("Не найдено: {0}")]
  NotFound(String),

  #[error("Ошибка валидации: {0}")]
  Validation(String),

  #[error("Внутренняя ошибка: {0}")]
  Internal(String),

  #[error("Не удалось получить доступ к БД (Mutex poisoned)")]
  LockError,
}

impl Serialize for AppError {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    use serde::ser::SerializeStruct;

    let (error_type, message) = match self {
      AppError::Database(e) => ("database", e.to_string()),
      AppError::NotFound(msg) => ("not_found", msg.clone()),
      AppError::Validation(msg) => ("validation", msg.clone()),
      AppError::Internal(msg) => ("internal", msg.clone()),
      AppError::LockError => ("lock_error", "Не удалось получить доступ к БД".to_string()),
    };

    let mut state = serializer.serialize_struct("AppError", 2)?;
    state.serialize_field("type", error_type)?;
    state.serialize_field("message", &message)?;
    state.end()
  }
}
