use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Priority {
  pub uuid: String,
  pub project_uuid: String,
  pub name: String,
  pub color: String,
  pub level: i32,
  pub created_at: String,
  pub updated_at: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePriority {
  pub project_uuid: String,
  pub name: String,
  #[serde(default = "default_color")]
  pub color: String,
  #[serde(default)]
  pub level: i32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePriority {
  pub name: Option<String>,
  pub color: Option<String>,
  pub level: Option<i32>,
}

fn default_color() -> String {
  "#6366f1".to_string()
}
