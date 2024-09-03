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
  ws.on_upgrade(| socket | handle_socket(socket, app))
}

async fn handle_socket( ws: WebSocket, app: Arc<AppHandler> ){
  let (
    ws_tx,
    mut ws_rx
  ) = ws.split();

  let msg = ws_rx.next().await.unwrap();
  if msg.is_err(){
    return;
  }

  let msg = msg.unwrap();

  match msg{
    Message::Close(_) => {
      return
    }
    Message::Text(msg) => {
      let msg: Value = serde_json::from_str(&msg).unwrap();

      match msg["type"].as_str().unwrap(){
        "auth" => {
          let verify = app.auth().verify_token(msg["token"].as_str().unwrap().to_owned()).await;
          if verify.is_some(){
            let ws_tx = Arc::new(Mutex::new(ws_tx));
            let mut app_rx = app.live().tx.lock().await.subscribe();

            tokio::spawn(async move {
              while let Ok(msg) = app_rx.recv().await{
                if ws_tx.lock().await.send(msg).await.is_err(){
                  break;
                }
              }
            });
          }
        }
        _ => {}
      }
    }
    _ => {}
  }

  while let Some(msg) = ws_rx.next().await{
    if msg.is_err(){
      return;
    }
  
    let msg = msg.unwrap();

    match msg{
      Message::Close(_) => {
        break;
      }
      _ => {}
    }
  }
}