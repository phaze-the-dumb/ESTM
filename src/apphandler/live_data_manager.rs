use std::sync::Arc;
use axum::extract::ws::Message;
use tokio::sync::{broadcast::{self, Sender}, Mutex};

#[derive(Debug, Clone)]
pub struct LiveDataManager{
  pub tx: Arc<Mutex<Sender<Message>>>
}

impl LiveDataManager{
  pub fn new() -> Self{
    let ( tx, _ ) = broadcast::channel(32);

    Self {
      tx: Arc::new(Mutex::new(tx))
    }
  }
}