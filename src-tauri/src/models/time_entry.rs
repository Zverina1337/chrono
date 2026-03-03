use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeEntry {
  pub uuid: String,
  pub task_uuid: String,
  pub started_at: String,
  pub ended_at: Option<String>,
  pub duration_minutes: Option<i32>,
  pub description: String,
  pub created_at: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTimeEntry {
  pub task_uuid: String,
  pub started_at: String,
  pub ended_at: Option<String>,
  pub duration_minutes: Option<i32>,
  #[serde(default)]
  pub description: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTimeEntry {
  pub ended_at: Option<Option<String>>,
  pub duration_minutes: Option<Option<i32>>,
  pub description: Option<String>,
}
