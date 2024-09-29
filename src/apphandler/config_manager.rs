use bson::doc;
use mongodb::Collection;

use crate::structs::config::{AppState, Config};

#[derive(Debug, Clone)]
pub struct ConfigManager{
  config: Collection<Config>
}

impl ConfigManager{
  pub fn new( config: Collection<Config> ) -> Self{
    ConfigManager { config } // Create new config manager instance
  }

  pub async fn get( &self ) -> Config{
    // Get the config data from the database
    let conf = self.config.find_one(doc! { "_id": "config" }).await.unwrap();

    if conf.is_none(){
      // We can't find a config object, so we'll create a new one
      let conf = Config {
        _id: "config".into(), // Set the _id to "config"
        current_match: "".into(), // There is no match currently selected, so this will be empty
        current_state: AppState::EDITING, // The default state is editing
        current_bracket_set: 0, // This can be set to whatever, it is updated when the match is started
        current_bracket: 0, // This can be set to whatever, it is updated when the match is started
        round_winner: 0 // This can be set to whatever, it is updated when the match is won
      };

      self.config.insert_one(&conf).await.unwrap(); // Add our new config data to the database
      return conf; // Return the config data
    }

    return conf.unwrap(); // Return the config data
  }

  pub async fn update( &self, conf: bson::Document ){
    // Allow other scripts to update values of the config data
    self.config.update_one(doc! { "_id": "config" }, doc! { "$set": conf }).await.unwrap();
  }
}