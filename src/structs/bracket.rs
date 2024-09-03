#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct Bracket{
  pub _id: String,
  pub team1: i32,
  pub team2: i32,
  pub winner: i32,
  pub match_id: String
}