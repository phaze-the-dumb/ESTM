#[derive(Debug, serde::Deserialize, serde::Serialize, PartialEq, Clone)]
pub enum AppState{
  EDITING,
  PLAYING
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct Config{
  pub _id: String,
  pub current_match: String,
  pub current_state: AppState,
  pub current_bracket_set: u32,
  pub current_bracket: u32,
  pub round_winner: u32
}