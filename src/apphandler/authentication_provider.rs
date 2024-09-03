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
    AuthenticationProvider { sessions }
  }

  pub async fn verify_token( &self, bearer: String ) -> Option<Session>{
    let session = self.sessions.find_one(doc! { "token": bearer }).await.unwrap();
    if session.is_none(){ return None }

    let session = session.unwrap();
    let timestamp = chrono::Utc::now().timestamp() as u64;

    if timestamp > session.expires_on{
      self.sessions.delete_one(doc! { "_id": session._id }).await.unwrap();
      None
    } else{
      Some(session)
    }
  }

  pub async fn new_session( &self ) -> ( String, String ){
    let token: String = thread_rng()
      .sample_iter(&Alphanumeric)
      .take(128)
      .map(char::from)
      .collect();

    let timestamp = chrono::Utc::now().timestamp() as u64;
    let id = nanoid!();

    self.sessions.insert_one(Session {
      _id: id.clone(),
      token: token.clone(),
      expires_on: timestamp + 86400
    }).await.unwrap();

    ( token, id )
  }

  pub async fn clean_sessions( &self ){
    let timestamp = chrono::Utc::now().timestamp() as u32;
    self.sessions.delete_many(doc! { "expires_on": { "$lt": timestamp } }).await.unwrap();
  }
}