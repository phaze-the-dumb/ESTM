use std::sync::Arc;
use axum::{ extract::ws::Message, http::{header, HeaderMap, StatusCode}, response::IntoResponse, Extension, Json };
use bson::doc;
use serde_json::json;
use crate::{apphandler::AppHandler, structs::config::AppState, util};

#[axum::debug_handler]
pub async fn post(
  Extension(app): Extension<Arc<AppHandler>>,
  headers: HeaderMap
) -> impl IntoResponse{
  // Check that the user has the correct permissions to start the match.
  let res = util::verify_auth::verify_auth(&headers, &app).await;
  if res.is_err(){ return res.unwrap_err(); }

  let mut config = app.config().get().await; // Get the current config data.
  match config.current_state{
    AppState::EDITING => {
      if config.current_match.is_empty(){
        // There is no match currently selected, we cannot start.
        return (
          StatusCode::OK,
          [
            ( header::ACCESS_CONTROL_ALLOW_ORIGIN, "*" ),
            ( header::ACCESS_CONTROL_ALLOW_METHODS, "POST" ),
            ( header::ACCESS_CONTROL_ALLOW_HEADERS, "Authorization,Content-Type" )
          ],
          Json(json!({ "ok": false, "error": "No match selected, Please selected a match and then try again." }))
        );
      }

      // Lets get the last bracket of set 1, the index of this bracket will be the amount of teams minus 1
      let bracket_index = app.teams().count(config.current_match.clone()).await as u32 - 1;

      // Update the global config value
      app.config().update(doc! { "current_state": "PLAYING", "current_bracket_set": 0, "current_bracket": bracket_index }).await;

      // Update our local config value
      config.current_bracket_set = 0;
      config.current_bracket = bracket_index;

      // Get the current session id
      let user_id = res.unwrap()._id;

      // Broadcast the "start-match" message to all clients
      let _ = app.live().tx.lock().await.send(Message::Text(json!({ "type": "start-match", "from": user_id.clone() }).to_string()));

      // Get the next bracket data. The current bracket will be empty as none of the clients have said that the teams are ready yet.
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

      // Update all clients current brackets and next brackets
      let _ = app.live().tx.lock().await.send(Message::Text(json!({ "type": "current-bracket", "bracket": { }, "from": user_id }).to_string()));
      let _ = app.live().tx.lock().await.send(Message::Text(json!({ "type": "next-bracket", "bracket": { "team1": next_bracket.0, "team2": next_bracket.1 }, "from": user_id }).to_string()));

      // Tell the client everything was successful
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
      // We can't start the match because the match is already started
      (
        StatusCode::OK,
        [
          ( header::ACCESS_CONTROL_ALLOW_ORIGIN, "*" ),
          ( header::ACCESS_CONTROL_ALLOW_METHODS, "POST" ),
          ( header::ACCESS_CONTROL_ALLOW_HEADERS, "Authorization,Content-Type" )
        ],
        Json(json!({ "ok": false, "error": "Not in editing mode, Please exit any current games and then try again" }))
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