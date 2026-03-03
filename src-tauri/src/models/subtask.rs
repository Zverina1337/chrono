use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Subtask {
  pub uuid: String,
  pub task_uuid: String,
  pub name: String,
  pub is_completed: bool,
  pub position: i32,
  pub created_at: String,
  pub updated_at: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSubtask {
  pub task_uuid: String,
  pub name: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSubtask {
  pub name: Option<String>,
  pub is_completed: Option<bool>,
  pub position: Option<i32>,
}
