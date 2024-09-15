use bson::doc;
use mongodb::Collection;

use crate::{structs::bracket::Bracket, util};

#[derive(Debug, Clone)]
pub struct BracketManager{
  brackets: Collection<Bracket>
}

impl BracketManager{
  pub fn new( brackets: Collection<Bracket> ) -> Self{
    Self{ brackets }
  }

  pub async fn generate( &self, team_count: u8, match_id: String ){
    self.brackets.delete_many(doc! { "match_id": &match_id }).await.unwrap();

    let brackets = util::generate_base_brackets::generate_base_brackets(team_count, match_id);
    self.brackets.insert_many(brackets).await.unwrap();
  }

  pub async fn list_brackets_in_match( &self, match_id: String ) -> Vec<Bracket>{
    let mut cursor = self.brackets.find(doc! { "match_id": match_id }).await.unwrap();
    let mut brackets = Vec::new();

    while cursor.advance().await.unwrap() {
      brackets.push(cursor.deserialize_current().unwrap()); }

    brackets
  }

  pub async fn delete_in_match( &self, match_id: String ){
    self.brackets.delete_many(doc! { "match_id": match_id }).await.unwrap();
  }

  pub async fn get_bracket( &self, bracket_set: u32, bracket: u32, match_id: String ) -> Option<Bracket>{
    self.brackets.find_one(doc! { "_id": format!("{}:{}:{}", bracket_set, bracket, match_id) }).await.unwrap()
  }

  pub async fn set_winner( &self, bracket_set: u32, bracket: u32, match_id: String, winner: u32 ){
    self.brackets.update_one(
      doc! { "_id": format!("{}:{}:{}", bracket_set, bracket, match_id) },
      doc! { "$set": { "winner": winner } }
    ).await.unwrap();
  }
}