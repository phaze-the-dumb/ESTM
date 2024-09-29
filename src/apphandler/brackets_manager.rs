use bson::doc;
use mongodb::Collection;

use crate::{structs::bracket::Bracket, util};

#[derive(Debug, Clone)]
pub struct BracketManager{
  brackets: Collection<Bracket>
}

impl BracketManager{
  pub fn new( brackets: Collection<Bracket> ) -> Self{
    Self { brackets }
  }

  pub async fn generate( &self, team_count: u8, match_id: String ){
    // Delete any brackets with the same match id.
    self.brackets.delete_many(doc! { "match_id": &match_id }).await.unwrap();

    // Generate a new set of brackets and insert them into the database.
    let brackets = util::generate_base_brackets::generate_base_brackets(team_count, match_id);
    self.brackets.insert_many(brackets).await.unwrap();
  }

  pub async fn list_brackets_in_match( &self, match_id: String ) -> Vec<Bracket>{
    // Get all brackets with this match_id in them
    let mut cursor = self.brackets.find(doc! { "match_id": match_id }).await.unwrap();
    let mut brackets = Vec::new(); // Initialise an empty Vec

    while cursor.advance().await.unwrap() { // Loop throught the cursor and fill up the Vec
      brackets.push(cursor.deserialize_current().unwrap()); }

    brackets // Return the Vec with all the brackets in it
  }

  pub async fn delete_in_match( &self, match_id: String ){
    // Delete any brackets with this match_id
    self.brackets.delete_many(doc! { "match_id": match_id }).await.unwrap();
  }

  pub async fn get_bracket( &self, bracket_set: u32, bracket: u32, match_id: String ) -> Option<Bracket>{
    // Bracket id's are laid out as "bracket set index:bracket index:match id", so we'll create an id based
    // on the provided args and then look that up in the database, which we'll then return
    self.brackets.find_one(doc! { "_id": format!("{}:{}:{}", bracket_set, bracket, match_id) }).await.unwrap()
  }

  pub async fn set_winner( &self, bracket_set: u32, bracket: u32, match_id: String, winner: u32 ){
    // Bracket id's are laid out as "bracket set index:bracket index:match id", so we'll create an id based
    // on the provided args and then look that up in the database and edit the "winner" value to be the
    // winner that has been provided in the args

    self.brackets.update_one(
      doc! { "_id": format!("{}:{}:{}", bracket_set, bracket, match_id) },
      doc! { "$set": { "winner": winner } }
    ).await.unwrap();
  }

  pub async fn reset_all( &self, match_id: String ){
    // List all brackets with this match_id
    let brackets = self.list_brackets_in_match(match_id).await;

    for bracket in brackets{    // For every bracket
      if !bracket._id.starts_with("0:"){ // if they aren't in the first set, then reset the winner to -1
        self.brackets.update_one(doc! { "_id": bracket._id }, doc! { "$set": { "winner": -1 } }).await.unwrap();
      }
    }
  }
}