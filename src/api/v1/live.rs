use std::sync::Arc;
use axum::{ extract::ws::{ WebSocket, WebSocketUpgrade, Message }, response::IntoResponse, Extension };
use serde_json::Value;
use tokio::sync::Mutex;
use crate::apphandler::AppHandler;
use futures_util::{ SinkExt, StreamExt };

pub async fn get(
  Extension(app): Extension<Arc<AppHandler>>,
  ws: WebSocketUpgrade
) -> impl IntoResponse{
  // When the client requests an upgrade, upgrade it to a socket
  ws.on_upgrade(| socket | handle_socket(socket, app))
}

async fn handle_socket( ws: WebSocket, app: Arc<AppHandler> ){
  let (
    ws_tx,
    mut ws_rx
  ) = ws.split();
  // Split the websocket into a sending and receiving channel.

  // Wait for the first message, this should be an authentication request.
  let msg = ws_rx.next().await.unwrap();
  if msg.is_err(){
    return; // If it's an error, return and close the socket.
  }

  let msg = msg.unwrap();

  match msg{
    Message::Text(msg) => {
      // The client has sent us text data. This is what we want
      // Decode the text to json
      let msg: Value = serde_json::from_str(&msg).unwrap();

      // Check the message type is an authentication request
      match msg["type"].as_str().unwrap(){
        "auth" => {
          // It is an auth request, lets verify the token they have sent to us.
          let verify = app.auth().verify_token(msg["token"].as_str().unwrap().to_owned()).await;

          if verify.is_some(){
            // It is a valid token, we'll subscribe them to the main broadcast channel
            let ws_tx = Arc::new(Mutex::new(ws_tx));
            let mut app_rx = app.live().tx.lock().await.subscribe();

            // Start a new thread to listen for messages from the broadcast channel and forward them to the client
            tokio::spawn(async move {
              while let Ok(msg) = app_rx.recv().await{
                if ws_tx.lock().await.send(msg).await.is_err(){
                  break;
                }
              }
            });
          } else{
            // They provided an invalid token, we'll just disconnect them for now
            return;
          }
        }
        _ => {
          // The first message isn't a text message this is incorrect for this
          // protocol, we'll just disconnect them.
          return;
        }
      }
    }
    _ => { return; }
  }

  // Here we are listening to messages from the client this
  // solely exists to check if the client has closed the connection
  while let Some(msg) = ws_rx.next().await{
    if msg.is_err(){
      // The client has sent us an error, we'll disconnect them
      return;
    }

    let msg = msg.unwrap();

    match msg{
      Message::Close(_) => {
        // The client has disconnected from us.
        break;
      }
      _ => {}
    }
  }
}