use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Project {
  pub uuid: String,
  pub name: String,
  pub description: String,
  pub position: i32,
  pub created_at: String,
  pub updated_at: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateProject {
  pub name: String,
  #[serde(default)]
  pub description: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateProject {
  pub name: Option<String>,
  pub description: Option<String>,
  pub position: Option<i32>,
}
