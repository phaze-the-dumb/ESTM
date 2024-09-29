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

use crate::{db_controller::DBController, structs::{bracket::Bracket, config::Config, team::Team}};

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
    let db_controller = DBController::new(); // Create a new instance of the db controller
    let options = ClientOptions::parse(db_controller.get_connection_uri()).await.unwrap(); // Create new Database connection options

    let client = Client::with_options(options).unwrap(); // Connect to the database using the connection options
    let db = client.database("ESTM"); // Get the ESTM database

    let auth_code: String = thread_rng() // Generate a random sequence of 6 characters
      .sample_iter(&Alphanumeric)
      .take(6)
      .map(char::from)
      .collect();

    println!("Authenticate using this code: {}", auth_code); // Print this code to the terminal

    Arc::new(AppHandler { // Create a new instance of the AppHandler struct and wrap it in an arc so it is thread safe
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

  pub async fn get_next_bracket_indexes( &self, config: &Config) -> Result<( u32, u32 ), &'static str>{
    let bracket_left_in_set = self.brackets.get_bracket(config.current_bracket_set, config.current_bracket + 1, config.current_match.clone()).await.is_some();

    let mut next_bracket_set = config.current_bracket_set;
    let mut next_bracket = config.current_bracket;

    if bracket_left_in_set{
      next_bracket += 1;
    } else{
      let bracket_in_next_set = self.brackets.get_bracket(config.current_bracket_set + 1, 0, config.current_match.clone()).await.is_some();

      if bracket_in_next_set {
        next_bracket_set += 1;
        next_bracket = 0;
      } else{
        return Err("No brackets left");
      }
    }

    Ok(( next_bracket_set, next_bracket ))
  }

  pub async fn get_next_bracket( &self, config: &Config ) -> Result<( Option<Team>, Option<Team> ), &'static str>{
    let bracket_left_in_set = self.brackets.get_bracket(config.current_bracket_set, config.current_bracket + 1, config.current_match.clone()).await.is_some();

    let mut next_bracket_set = config.current_bracket_set;
    let mut next_bracket = config.current_bracket;

    if bracket_left_in_set{
      next_bracket += 1;
    } else{
      let bracket_in_next_set = self.brackets.get_bracket(config.current_bracket_set + 1, 0, config.current_match.clone()).await.is_some();

      if bracket_in_next_set {
        next_bracket_set += 1;
        next_bracket = 0;
      } else{
        return Ok(( None, None ))
      }
    }

    let bracket = self.brackets.get_bracket(next_bracket_set, next_bracket, config.current_match.clone()).await;

    if bracket.is_none(){ return Ok(( None, None )) }

    let teams = self.teams.list_teams_in_match(config.current_match.clone()).await;
    let bracket = bracket.unwrap();

    let team1 = teams[bracket.team1 as usize].clone();

    if bracket.team2 == -1{
      dbg!("bracket 1 help");
      return Err("Team 2 does not exist");
    }

    let team2 = teams[bracket.team2 as usize].clone();

    Ok(( Some(team1), Some(team2) ))
  }

  pub async fn get_current_bracket( &self, config: &Config ) -> Result<( Option<Team>, Option<Team>, u8 ), &'static str>{
    dbg!(config);
    let bracket = self.brackets.get_bracket(config.current_bracket_set, config.current_bracket, config.current_match.clone()).await;

    if bracket.is_none(){ return Ok(( None, None, 0 )) }

    let teams = self.teams.list_teams_in_match(config.current_match.clone()).await;
    let bracket = bracket.unwrap();
    dbg!(&bracket);

    if bracket.team2 == -1{
      // TODO: Handle half brackets
      dbg!("bracket 1 help");
      return Err("Team 2 does not exist");
    }

    let team1_bracket = self.brackets.get_bracket(config.current_bracket_set - 1, bracket.team1 as u32, config.current_match.clone()).await.unwrap();
    let team1 = teams[team1_bracket.winner as usize].clone();

    let team2_bracket = self.brackets.get_bracket(config.current_bracket_set - 1, bracket.team2 as u32, config.current_match.clone()).await.unwrap();
    let team2 = teams[team2_bracket.winner as usize].clone();

    if bracket.winner != -1{
      let winner = &teams[bracket.winner as usize];

      if winner._id == team1._id{
        return Ok(( Some(team1), Some(team2), 1 ))
      } else if winner._id == team2._id{
        return Ok(( Some(team1), Some(team2), 2 ))
      }
    }

    Ok(( Some(team1), Some(team2), 0 ))
  }
}