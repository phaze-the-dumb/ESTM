use bson::doc;
use mongodb::Collection;
use nanoid::nanoid;

use crate::structs::game::Game;

#[derive(Debug, Clone)]
pub struct MatchController{
  matches: Collection<Game>
}

impl MatchController{
  pub fn new( matches: Collection<Game> ) -> Self{
    Self { matches }
  }

  pub async fn get( &self, id: &String ) -> Option<Game>{
    self.matches.find_one(doc! { "_id": id }).await.unwrap()
  }

  pub async fn create_match( &self, name: String ) -> String{
    let _id = nanoid!();
    self.matches.insert_one(Game { _id: _id.clone(), name }).await.unwrap();

    _id
  }

  pub async fn list_matches( &self ) -> Vec<Game>{
    let mut cursor = self.matches.find(doc! {}).await.unwrap();
    let mut matches = Vec::new();

    while cursor.advance().await.unwrap() {
      matches.push(cursor.deserialize_current().unwrap()); }

    matches
  }

  pub async fn rename( &self, id: String, name: String ){
    self.matches.update_one(doc! { "_id": id }, doc! { "$set": { "name": name } }).await.unwrap();
  }

  pub async fn delete( &self, id: String ){
    self.matches.delete_one(doc! { "_id": id }).await.unwrap();
  }
}