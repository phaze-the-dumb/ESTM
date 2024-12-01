use super::player::Player;

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct Team{
  pub _id: String,
  pub name: String,
  pub match_id: String,
  pub colour: String,
  pub players: Vec<Player>
}