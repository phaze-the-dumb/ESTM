use std::sync::Arc;
use axum::{ extract::ws::Message, http::{header, HeaderMap, StatusCode}, response::IntoResponse, Extension, Json };
use bson::doc;
use serde_json::{json, Value};
use crate::{apphandler::AppHandler, util, structs::config::AppState};

#[axum::debug_handler]
pub async fn put(
  Extension(app): Extension<Arc<AppHandler>>,
  headers: HeaderMap,
  Json(body): Json<Value>
) -> impl IntoResponse{
  // Verify that this session has permission to use this endpoint
  let res = util::verify_auth::verify_auth(&headers, &app).await;
  if res.is_err(){ return res.unwrap_err(); }

  let config = app.config().get().await; // Get current config
  match config.current_state{ // Check the current app state
    AppState::PLAYING => {
      // Get the winning team from the request body
      let team = body["team"].as_str().unwrap();
      // Get current bracket data
      let current_bracket = app.brackets().get_bracket(config.current_bracket_set, config.current_bracket, config.current_match.clone()).await.unwrap();

      match team{
        "team1" => {
          // The client has told us team1 has won.

          // Get the index of team1
          let winner = app.brackets().get_bracket(config.current_bracket_set - 1, current_bracket.team1 as u32, config.current_match.clone()).await.unwrap().winner;
          // Set them as the winner
          app.brackets().set_winner(config.current_bracket_set, config.current_bracket, config.current_match.clone(), winner as u32).await;
        }
        "team2" => {
          // The client has told us team2 has won.

          // Get the index of team2
          let winner = app.brackets().get_bracket(config.current_bracket_set - 1, current_bracket.team2 as u32, config.current_match.clone()).await.unwrap().winner;
          // Set them as the winner
          app.brackets().set_winner(config.current_bracket_set, config.current_bracket, config.current_match.clone(), winner as u32).await;
        }
        _ => {
          // The client did not send a valid team, send an error back
          return (
            StatusCode::OK,
            [
              ( header::ACCESS_CONTROL_ALLOW_ORIGIN, "*" ),
              ( header::ACCESS_CONTROL_ALLOW_METHODS, "PUT" ),
              ( header::ACCESS_CONTROL_ALLOW_HEADERS, "Authorization,Content-Type" )
            ],
            Json(json!({ "ok": false, "error": "Invalid team value" }))
          )
        }
      }

      // Get the current session id, we need to do this so we can clone the value to use it with multiple broadcasts.
      let user_id = res.unwrap()._id;
      // Broadcast the winning team
      let _ = app.live().tx.lock().await.send(Message::Text(json!({ "type": "win-bracket", "team": team, "from": user_id.clone() }).to_string()));

      // Get the next bracket
      let next_bracket = app.get_next_bracket(config).await;

      // There was an error trying to get the next bracket, We'll forward this error onto the client to display to the user
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

      let next_bracket = next_bracket.unwrap();
      // Update the clients as to what the next bracket is, under certain situations we cannot get the next bracket before knowing this information.
      let _ = app.live().tx.lock().await.send(Message::Text(json!({ "type": "next-bracket", "bracket": { "team1": next_bracket.0, "team2": next_bracket.1 }, "from": user_id }).to_string()));

      // Return success to the client
      (
        StatusCode::OK,
        [
          ( header::ACCESS_CONTROL_ALLOW_ORIGIN, "*" ),
          ( header::ACCESS_CONTROL_ALLOW_METHODS, "PUT" ),
          ( header::ACCESS_CONTROL_ALLOW_HEADERS, "Authorization,Content-Type" )
        ],
        Json(json!({ "ok": true }))
      )
    }
    _ => {
      // The app is not in playing mode, we cannot play.
      (
        StatusCode::OK,
        [
          ( header::ACCESS_CONTROL_ALLOW_ORIGIN, "*" ),
          ( header::ACCESS_CONTROL_ALLOW_METHODS, "PUT" ),
          ( header::ACCESS_CONTROL_ALLOW_HEADERS, "Authorization,Content-Type" )
        ],
        Json(json!({ "ok": false, "error": "Not in playing mode, Please start a match first" }))
      )
    }
  }
}

pub async fn options() -> impl IntoResponse{
  (
    StatusCode::OK,
    [
      ( header::ACCESS_CONTROL_ALLOW_ORIGIN, "*" ),
      ( header::ACCESS_CONTROL_ALLOW_METHODS, "PUT" ),
      ( header::ACCESS_CONTROL_ALLOW_HEADERS, "Authorization,Content-Type" )
    ],
    "200 OK"
  )
}