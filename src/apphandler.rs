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

  pub fn auth( &self ) -> &AuthenticationProvider{
    &self.auth // Return non-mutable reference to auth provider
  }

  pub fn matches( &self ) -> &MatchController{
    &self.matches // Return non-mutable reference to match controller
  }

  pub fn config( &self ) -> &ConfigManager{
    &self.config // Return non-mutable reference to config controller
  }

  pub fn live( &self ) -> &LiveDataManager{
    &self.live // Return non-mutable reference to live data controller
  }

  pub fn teams( &self ) -> &TeamManager{
    &self.teams // Return non-mutable reference to team controller
  }

  pub fn brackets( &self ) -> &BracketManager{
    &self.brackets // Return non-mutable reference to brackets controller
  }

  pub fn verify_code( &self, code: String ) -> bool{
    self.auth_code == code // Check if they're equal
  }

  pub async fn get_next_bracket_indexes( &self, config: &Config ) -> Result<( u32, u32 ), &'static str>{
    // Clone the config values so we can read / write to them
    let mut next_bracket_set = config.current_bracket_set;
    let mut next_bracket = config.current_bracket;

    // Is there a bracket left in this set
    let bracket_left_in_set = self.brackets.get_bracket(config.current_bracket_set, config.current_bracket + 1, config.current_match.clone()).await.is_some();

    if bracket_left_in_set{
      // There is a bracket left, we can just increment the bracket index by one and leave the set value the same
      next_bracket += 1;
    } else{
      // There's no bracket left in this set, we'll check the next set to see if there is a bracket there.
      let bracket_in_next_set = self.brackets.get_bracket(config.current_bracket_set + 1, 0, config.current_match.clone()).await.is_some();

      if bracket_in_next_set {
        // There is a bracket in this set, so we'll increment the set by one and reset the bracket index back to 0
        next_bracket_set += 1;
        next_bracket = 0;
      } else{
        // There are no more brackets left, this is the last bracket
        return Err("No brackets left");
      }
    }

    // Return the bracket set and bracket index
    Ok(( next_bracket_set, next_bracket ))
  }

  pub async fn get_next_bracket( &self, config: Config ) -> Result<( Option<Team>, Option<Team> ), &'static str>{
    // Find the next full bracket object
    let bracket = self.find_next_full_bracket(config.clone()).await;

    // There is no bracket found, we'll just return None for both teams
    if bracket.is_err(){
      return Ok(( None, None ))
    }

    // Unwrap the updated config data and the bracket object
    let ( config, bracket ) = bracket.unwrap();
    // Get all teams in this match
    let teams = self.teams.list_teams_in_match(config.current_match.clone()).await;

    // the "team1" value on the bracket object holds the index of a bracket in a previous set, we'll look for this bracket and then read the winner value from it
    let team1_bracket = self.brackets.get_bracket(config.current_bracket_set - 1, bracket.team1 as u32, config.current_match.clone()).await.unwrap();
    let team1;

    // If there is no winner for this bracket, we'll just return None for this team
    if team1_bracket.winner == -1{
      team1 = None;
    } else{
      // If there is a winner, we'll look for this team in the teams list and return that instead
      team1 = Some(teams[team1_bracket.winner as usize].clone());
    }

    // If there is no team2 value on the bracket object then throw an error
    if bracket.team2 == -1{
      // This should never, ever, ever happen. If you managed to get this error you have
      // managed to really mess something up.
      return Err("Team 2 does not exist");
    }

    // the "team2" value on the bracket object holds the index of a bracket in a previous set, we'll look for this bracket and then read the winner value from it
    let team2_bracket = self.brackets.get_bracket(config.current_bracket_set - 1, bracket.team2 as u32, config.current_match.clone()).await.unwrap();
    let team2;

    // If there is no winner for this bracket, we'll just return None for this team
    if team2_bracket.winner == -1{
      team2 = None;
    } else{
      // If there is a winner, we'll look for this team in the teams list and return that instead
      team2 = Some(teams[team2_bracket.winner as usize].clone());
    }

    // Return both teams
    Ok(( team1, team2 ))
  }

  pub async fn get_current_bracket( &self, config: &Config ) -> Result<( Option<Team>, Option<Team>, u8 ), &'static str>{
    // Get the current bracket
    let bracket = self.brackets.get_bracket(config.current_bracket_set, config.current_bracket, config.current_match.clone()).await;

    // There is no bracket here, so we'll just return None for both teams, and the 0 because there is no winner
    if bracket.is_none(){ return Ok(( None, None, 0 )) }

    // List all the teams in this match
    let teams = self.teams.list_teams_in_match(config.current_match.clone()).await;
    let bracket = bracket.unwrap();

    // If team2 does not exist on this bracket, throw an error
    if bracket.team2 == -1{
      // This should never, ever, ever happen. If you managed to get this error you have
      // managed to really mess something up.
      return Err("Team 2 does not exist");
    }

    // the "team1" value on the bracket object holds the index of a bracket in a previous set, we'll look for this bracket and then read the winner value from it
    let team1_bracket = self.brackets.get_bracket(config.current_bracket_set - 1, bracket.team1 as u32, config.current_match.clone()).await.unwrap();
    let team1 = teams[team1_bracket.winner as usize].clone(); // Look the winner up in the list of teams

    // the "team2" value on the bracket object holds the index of a bracket in a previous set, we'll look for this bracket and then read the winner value from it
    let team2_bracket = self.brackets.get_bracket(config.current_bracket_set - 1, bracket.team2 as u32, config.current_match.clone()).await.unwrap();
    let team2 = teams[team2_bracket.winner as usize].clone(); // Look the winner up in the list of teams

    // If there is a winner for this bracket then we need to return that too
    if bracket.winner != -1{
      let winner = &teams[bracket.winner as usize];
      // Get the winning team

      if winner._id == team1._id{ // We'll compare the team ids and see if they're the same as team1 or team2
        return Ok(( Some(team1), Some(team2), 1 ))
      } else if winner._id == team2._id{
        return Ok(( Some(team1), Some(team2), 2 ))
      } else{
        // Technically this can be some other value but this application will never set it to anything else, if winner._id doesn't equal
        // either of the above values that means someone has tampered with the database.
        return Ok(( Some(team1), Some(team2), 0 ))
      }
    }

    // Return the teams
    Ok(( Some(team1), Some(team2), 0 ))
  }

  pub async fn find_next_full_bracket( &self, config: Config ) -> Result<( Config, Bracket ), &'static str>{
    // Clone the config data so we can edit it
    let mut config = config;

    // We'll keep running this code until we find a bracket with two teams, we'll break out when we find a bracket we're looking for
    loop{
      let mut next_bracket_set = config.current_bracket_set;
      let mut next_bracket = config.current_bracket;

      // Is there a bracket left in this set
      let bracket_left_in_set = self.brackets.get_bracket(config.current_bracket_set, config.current_bracket + 1, config.current_match.clone()).await.is_some();

      if bracket_left_in_set{
        // There is a bracket left, we can just increment the bracket index by one and leave the set value the same
        next_bracket += 1;
      } else{
        // There's no bracket left in this set, we'll check the next set to see if there is a bracket there.
        let bracket_in_next_set = self.brackets.get_bracket(config.current_bracket_set + 1, 0, config.current_match.clone()).await.is_some();

        if bracket_in_next_set {
          // There is a bracket in this set, so we'll increment the set by one and reset the bracket index back to 0
          next_bracket_set += 1;
          next_bracket = 0;
        } else{
          // There are no more brackets left, this is the last bracket
          return Err("Cannot find next bracket")
        }
      }

      // Get
      let bracket = self.brackets.get_bracket(next_bracket_set, next_bracket, config.current_match.clone()).await.unwrap();

      config.current_bracket = next_bracket;
      config.current_bracket_set = next_bracket_set;

      if bracket.team2 != -1{
        break Ok(( config, bracket ));
      } else{
        let team = self.brackets.get_bracket(config.current_bracket_set - 1, bracket.team1 as u32, config.current_match.clone()).await.unwrap().winner;
        self.brackets.set_winner(next_bracket_set, next_bracket, config.current_match.clone(), team as u32).await;
      }
    }
  }
}