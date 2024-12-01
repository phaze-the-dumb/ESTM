use std::sync::Arc;
use axum::{ extract::ws::Message, http::{header, HeaderMap, StatusCode}, response::IntoResponse, Extension, Json };
use bson::doc;
use serde_json::json;
use crate::{apphandler::AppHandler, util, structs::config::AppState};

#[axum::debug_handler]
pub async fn post(
  Extension(app): Extension<Arc<AppHandler>>,
  headers: HeaderMap
) -> impl IntoResponse{
  // Verify that this session has permission to use this endpoint
  let res = util::verify_auth::verify_auth(&headers, &app).await;
  if res.is_err(){ return res.unwrap_err(); }

  let mut config = app.config().get().await; // Get current config
  match config.current_state{ // Check the current app state
    AppState::PLAYING => {
      // Get current bracket data
      let current_bracket = app.brackets().get_bracket(config.current_bracket_set, config.current_bracket, config.current_match.clone()).await.unwrap();

      // If there is no winner of this bracket set, then error and say that they need to select a winner
      if current_bracket.winner == -1{
        return (
          StatusCode::OK,
          [
            ( header::ACCESS_CONTROL_ALLOW_ORIGIN, "*" ),
            ( header::ACCESS_CONTROL_ALLOW_METHODS, "POST" ),
            ( header::ACCESS_CONTROL_ALLOW_HEADERS, "Authorization,Content-Type" )
          ],
          Json(json!({ "ok": false, "error": "You need to select a winner of the current match first" }))
        );
      }

      // Look for the next full bracket
      config = app.find_next_full_bracket(config).await.unwrap().0;
      // Update the config to have the next bracket selected
      app.config().update(doc! { "current_bracket": config.current_bracket, "current_bracket_set": config.current_bracket_set }).await;

      // Get the current session id, we need to do this so we can clone the value to use it with multiple broadcasts.
      let user_id = res.unwrap()._id;

      // Get the current and next bracket using the helper functions on AppHandler
      let current_bracket = app.get_current_bracket(&config).await;
      let next_bracket = app.get_next_bracket(config).await;

      // Check if there was an error getting the next bracket and return the error to the client
      if next_bracket.is_err(){
        return (
          StatusCode::OK,
          [
            ( header::ACCESS_CONTROL_ALLOW_ORIGIN, "*" ),
            ( header::ACCESS_CONTROL_ALLOW_METHODS, "POST" ),
            ( header::ACCESS_CONTROL_ALLOW_HEADERS, "Authorization,Content-Type" )
          ],
          Json(json!({ "ok": false, "error": next_bracket.unwrap_err() }))
        );
      }

      // Check if there was an error getting the current bracket and return the error to the client
      if current_bracket.is_err(){
        return (
          StatusCode::OK,
          [
            ( header::ACCESS_CONTROL_ALLOW_ORIGIN, "*" ),
            ( header::ACCESS_CONTROL_ALLOW_METHODS, "POST" ),
            ( header::ACCESS_CONTROL_ALLOW_HEADERS, "Authorization,Content-Type" )
          ],
          Json(json!({ "ok": false, "error": current_bracket.unwrap_err() }))
        );
      }

      let next_bracket = next_bracket.unwrap();
      let current_bracket = current_bracket.unwrap();

      // Tell all the clients what brackets are currently happening and which ones are next up
      let _ = app.live().tx.lock().await.send(Message::Text(json!({ "type": "current-bracket", "bracket": { "team1": current_bracket.0, "team2": current_bracket.1 }, "from": user_id }).to_string()));
      let _ = app.live().tx.lock().await.send(Message::Text(json!({ "type": "next-bracket", "bracket": { "team1": next_bracket.0, "team2": next_bracket.1 }, "from": user_id }).to_string()));

      // Return success to the client
      (
        StatusCode::OK,
        [
          ( header::ACCESS_CONTROL_ALLOW_ORIGIN, "*" ),
          ( header::ACCESS_CONTROL_ALLOW_METHODS, "POST" ),
          ( header::ACCESS_CONTROL_ALLOW_HEADERS, "Authorization,Content-Type" )
        ],
        Json(json!({ "ok": true }))
      )
    }
    _ => {
      (
        // The app isn't in play mode, they can't edit the current match state.
        StatusCode::OK,
        [
          ( header::ACCESS_CONTROL_ALLOW_ORIGIN, "*" ),
          ( header::ACCESS_CONTROL_ALLOW_METHODS, "POST" ),
          ( header::ACCESS_CONTROL_ALLOW_HEADERS, "Authorization,Content-Type" )
        ],
        Json(json!({ "ok": false, "error": "Not in play mode, Please start a match first" }))
      )
    }
  }
}

pub async fn options() -> impl IntoResponse{
  (
    StatusCode::OK,
    [
      ( header::ACCESS_CONTROL_ALLOW_ORIGIN, "*" ),
      ( header::ACCESS_CONTROL_ALLOW_METHODS, "POST" ),
      ( header::ACCESS_CONTROL_ALLOW_HEADERS, "Authorization,Content-Type" )
    ],
    "200 OK"
  )
}