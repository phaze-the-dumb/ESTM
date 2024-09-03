#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub enum AppState{
  EDITING
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Config{
  pub _id: String,
  pub current_match: String,
  pub current_state: AppState
}