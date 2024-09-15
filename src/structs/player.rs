#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct Player{
  pub _id: String,
  pub name: String
}