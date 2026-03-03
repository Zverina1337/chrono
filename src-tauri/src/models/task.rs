use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Task {
  pub uuid: String,
  pub project_uuid: String,
  pub status_uuid: Option<String>,
  pub priority_uuid: Option<String>,
  pub name: String,
  pub description: String,
  pub due_date: Option<String>,
  pub start_date: Option<String>,
  pub estimated_minutes: Option<i32>,
  pub position: i32,
  pub created_at: String,
  pub updated_at: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTask {
  pub project_uuid: String,
  pub status_uuid: Option<String>,
  pub priority_uuid: Option<String>,
  pub name: String,
  #[serde(default)]
  pub description: String,
  pub due_date: Option<String>,
  pub start_date: Option<String>,
  pub estimated_minutes: Option<i32>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTask {
  pub status_uuid: Option<Option<String>>,
  pub priority_uuid: Option<Option<String>>,
  pub name: Option<String>,
  pub description: Option<String>,
  pub due_date: Option<Option<String>>,
  pub start_date: Option<Option<String>>,
  pub estimated_minutes: Option<Option<i32>>,
  pub position: Option<i32>,
}
