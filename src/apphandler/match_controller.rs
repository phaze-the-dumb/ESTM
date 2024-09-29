use bson::doc;
use mongodb::Collection;
use nanoid::nanoid;

use crate::structs::game::Match;

#[derive(Debug, Clone)]
pub struct MatchController{
  matches: Collection<Match>
}

impl MatchController{
  pub fn new( matches: Collection<Match> ) -> Self{
    Self { matches }
  }

  pub async fn get( &self, id: &String ) -> Option<Match>{
    // Look for one match in the database with this id
    self.matches.find_one(doc! { "_id": id }).await.unwrap()
  }

  pub async fn create_match( &self, name: String ) -> String{
    let _id = nanoid!(); // Generate random id

    // Create a new match with this id and the provided name
    self.matches.insert_one(Match { _id: _id.clone(), name }).await.unwrap();
    _id // Return the generated id
  }

  pub async fn list_matches( &self ) -> Vec<Match>{
    // Get every match in the database
    let mut cursor = self.matches.find(doc! {}).await.unwrap();
    let mut matches = Vec::new(); // Create an empty Vec

    while cursor.advance().await.unwrap() { // Loop throught the cursor and fill up the Vec
      matches.push(cursor.deserialize_current().unwrap()); }

    matches // Return the Vec with all the matches in it
  }

  pub async fn rename( &self, id: String, name: String ){
    // Updates the name of one object in the database
    self.matches.update_one(doc! { "_id": id }, doc! { "$set": { "name": name } }).await.unwrap();
  }

  pub async fn delete( &self, id: String ){
    // Deletes one object from the database
    self.matches.delete_one(doc! { "_id": id }).await.unwrap();
  }
}