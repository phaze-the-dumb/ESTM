mod authentication_provider;
mod match_controller;
mod config_manager;
mod live_data_manager;
mod team_manager;
mod brackets_manager;

use std::sync::Arc;
use authentication_provider::AuthenticationProvider;
use brackets_manager::BracketManager;
use config_manager::ConfigManager;
use live_data_manager::LiveDataManager;
use match_controller::MatchController;
use mongodb::{ options::ClientOptions, Client };
use rand::{ distributions::Alphanumeric, thread_rng, Rng };
use team_manager::TeamManager;

use crate::db_controller::DBController;

#[derive(Debug, Clone)]
pub struct AppHandler {
  auth_code: String,
  auth: AuthenticationProvider,
  matches: MatchController,
  config: ConfigManager,
  teams: TeamManager,
  live: LiveDataManager,
  brackets: BracketManager,
  db: DBController,
}

impl AppHandler{
  pub async fn new() -> Arc<Self>{
    let db_controller = DBController::new();
    let options = ClientOptions::parse(db_controller.get_connection_uri()).await.unwrap();

    let client = Client::with_options(options).unwrap();
    let db = client.database("ESTM");

    let auth_code: String = thread_rng()
      .sample_iter(&Alphanumeric)
      .take(6)
      .map(char::from)
      .collect();

    println!("Authenticate using this code: {}", auth_code);

    Arc::new(AppHandler {
      auth_code,
      auth: AuthenticationProvider::new(db.collection("Sessions")),
      matches: MatchController::new(db.collection("Matches")),
      config: ConfigManager::new(db.collection("Config")),
      teams: TeamManager::new(db.collection("Teams")),
      brackets: BracketManager::new(db.collection("Brackets")),
      live: LiveDataManager::new(),
      db: db_controller
    })
  }

  pub fn verify_code( &self, code: String ) -> bool{
    self.auth_code == code
  }

  pub fn auth( &self ) -> &AuthenticationProvider{
    &self.auth
  }

  pub fn matches( &self ) -> &MatchController{
    &self.matches
  }

  pub fn config( &self ) -> &ConfigManager{
    &self.config
  }

  pub fn live( &self ) -> &LiveDataManager{
    &self.live
  }

  pub fn teams( &self ) -> &TeamManager{
    &self.teams
  }

  pub fn brackets( &self ) -> &BracketManager{
    &self.brackets
  }
}