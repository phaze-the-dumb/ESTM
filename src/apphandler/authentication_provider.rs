use bson::doc;
use mongodb::Collection;
use nanoid::nanoid;
use rand::{ distributions::Alphanumeric, thread_rng, Rng };

use crate::structs::session::Session;

#[derive(Debug, Clone)]
pub struct AuthenticationProvider{
  sessions: Collection<Session>
}

impl AuthenticationProvider{
  pub fn new( sessions: Collection<Session> ) -> Self{
    // Create a new instance of the authentication provider and
    // embed the "sessions" collection of the database into it
    AuthenticationProvider { sessions }
  }

  pub async fn verify_token( &self, bearer: String ) -> Option<Session>{
    // Look the session up in the database by the provided token
    let session = self.sessions.find_one(doc! { "token": bearer }).await.unwrap();
    if session.is_none(){ return None } // That token does not exist in the database

    let session = session.unwrap();
    let timestamp = chrono::Utc::now().timestamp() as u64; // Get current timestamp

    if timestamp > session.expires_on{ // Has the session expired
      // The session has expired, so we'll delete the session and return None
      self.sessions.delete_one(doc! { "_id": session._id }).await.unwrap();
      None
    } else{
      // The session is still valid, so we'll return the session.
      Some(session)
    }
  }

  pub async fn new_session( &self ) -> ( String, String ){
    let token: String = thread_rng() // Generate random alpha-numeric data, we'll use this as a token
      .sample_iter(&Alphanumeric)
      .take(128)
      .map(char::from)
      .collect();

    let timestamp = chrono::Utc::now().timestamp() as u64; // Get current timestamp
    let id = nanoid!(); // Generate a random id for this session

    self.sessions.insert_one(Session {
      _id: id.clone(), // Set this sessions id
      token: token.clone(), // Set this sessions token
      expires_on: timestamp + 86400 // Set the expires_on value to the current timestamp plus one day ( in seconds )
    }).await.unwrap();

    ( token, id ) // Return the token and the id
  }

  pub async fn clean_sessions( &self ){
    let timestamp = chrono::Utc::now().timestamp() as u32; // Get current timestamp

    // Delete any sessions that have their expires_on value set to less than the current timestamp
    self.sessions.delete_many(doc! { "expires_on": { "$lt": timestamp } }).await.unwrap();
  }
}