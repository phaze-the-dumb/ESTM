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
}