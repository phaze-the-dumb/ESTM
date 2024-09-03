use bson::doc;
use mongodb::Collection;

use crate::structs::config::{AppState, Config};

#[derive(Debug, Clone)]
pub struct ConfigManager{
  config: Collection<Config>
}

impl ConfigManager{
  pub fn new( config: Collection<Config> ) -> Self{
    ConfigManager { config }
  }

  pub async fn get( &self ) -> Config{
    let conf = self.config.find_one(doc! { "_id": "config" }).await.unwrap();

    if conf.is_none(){
      let conf = Config {
        _id: "config".into(),
        current_match: "".into(),
        current_state: AppState::EDITING
      };

      self.config.insert_one(&conf).await.unwrap();
      return conf;
    }

    return conf.unwrap();
  }

  pub async fn update( &self, conf: bson::Document ){
    self.config.update_one(doc! { "_id": "config" }, doc! { "$set": conf }).await.unwrap();
  }
}