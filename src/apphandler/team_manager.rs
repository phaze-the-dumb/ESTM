use bson::doc;
use mongodb::Collection;
use nanoid::nanoid;

use crate::structs::team::Team;

#[derive(Debug, Clone)]
pub struct TeamManager{
  teams: Collection<Team>
}

impl TeamManager{
  pub fn new( teams: Collection<Team> ) -> Self{
    Self { teams }
  }

  pub async fn create( &self, name: String, match_id: String ) -> String{
    let _id = nanoid!(); // Generate a random id

    // Create a new team and insert it into the database
    self.teams.insert_one(Team { _id: _id.clone(), name, players: Vec::new(), match_id }).await.unwrap();
    _id
  }

  pub async fn list_teams_in_match( &self, match_id: String ) -> Vec<Team>{
    let mut cursor = self.teams.find(doc! { "match_id": match_id }).await.unwrap();
    let mut teams = Vec::new();

    while cursor.advance().await.unwrap() {
      teams.push(cursor.deserialize_current().unwrap()); }

    teams
  }

  pub async fn rename( &self, id: String, name: String ){
    self.teams.update_one(doc! { "_id": id }, doc! { "$set": { "name": name } }).await.unwrap();
  }

  pub async fn delete( &self, id: String ) -> String{
    let team = self.teams.find_one(doc! { "_id": id.clone() }).await.unwrap().unwrap();
    self.teams.delete_one(doc! { "_id": id }).await.unwrap();

    team.match_id
  }

  pub async fn delete_in_match( &self, match_id: String ){
    // Delete every team with this match_id
    self.teams.delete_many(doc! { "match_id": match_id }).await.unwrap();
  }

  pub async fn count( &self, match_id: String ) -> u8{
    // Get mongo to count every team in the database with the provided match_id
    self.teams.count_documents(doc! { "match_id": match_id }).await.unwrap() as u8
  }
}