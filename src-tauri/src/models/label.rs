use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Label {
  pub uuid: String,
  pub name: String,
  pub color: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateLabel {
  pub name: String,
  #[serde(default = "default_color")]
  pub color: String,
}

fn default_color() -> String {
  "#6366f1".to_string()
}
