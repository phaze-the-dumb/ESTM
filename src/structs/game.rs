// This file is named "game" as "match" is a keyword in rust,
// I cannot name this file "match"

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Match{
  pub _id: String,
  pub name: String
}