use std::sync::Arc;
use axum::{ extract::ws::Message, http::{header, HeaderMap, StatusCode}, response::IntoResponse, Extension, Json };
use bson::doc;
use serde_json::json;
use crate::{apphandler::AppHandler, structs::{bracket, config::AppState}, util};

#[axum::debug_handler]
pub async fn post(
  Extension(app): Extension<Arc<AppHandler>>,
  headers: HeaderMap
) -> impl IntoResponse{
  let res = util::verify_auth::verify_auth(&headers, &app).await;
  if res.is_err(){ return res.unwrap_err(); }

  let mut config = app.config().get().await;
  match config.current_state{
    AppState::EDITING => {
      if config.current_match.is_empty(){
        return (
          StatusCode::OK,
          [
            ( header::ACCESS_CONTROL_ALLOW_ORIGIN, "http://localhost:5173" ),
            ( header::ACCESS_CONTROL_ALLOW_METHODS, "POST" ),
            ( header::ACCESS_CONTROL_ALLOW_HEADERS, "Authorization,Content-Type" )
          ],
          Json(json!({ "ok": false, "error": "No match selected, Please selected a match and then try again." }))
        );
      }

      let bracket_index = app.teams().count(config.current_match.clone()).await as u32 - 1;
      app.config().update(doc! { "current_state": "PLAYING", "current_bracket_set": 0, "current_bracket": bracket_index }).await;

      config.current_bracket_set = 0;
      config.current_bracket = bracket_index;

      let user_id = res.unwrap()._id;

      let _ = app.live().tx.lock().await.send(Message::Text(json!({ "type": "start-match", "from": user_id.clone() }).to_string()));

      let next_bracket = app.get_next_bracket(&config).await;

      if next_bracket.is_err(){
        return (
          StatusCode::OK,
          [
            ( header::ACCESS_CONTROL_ALLOW_ORIGIN, "http://localhost:5173" ),
            ( header::ACCESS_CONTROL_ALLOW_METHODS, "POST" ),
            ( header::ACCESS_CONTROL_ALLOW_HEADERS, "Authorization,Content-Type" )
          ],
          Json(json!({ "ok": false, "error": next_bracket.unwrap_err() }))
        );
      }

      let next_bracket = next_bracket.unwrap();

      let _ = app.live().tx.lock().await.send(Message::Text(json!({ "type": "current-bracket", "bracket": { }, "from": user_id }).to_string()));
      let _ = app.live().tx.lock().await.send(Message::Text(json!({ "type": "next-bracket", "bracket": { "team1": next_bracket.0, "team2": next_bracket.1 }, "from": user_id }).to_string()));

      (
        StatusCode::OK,
        [
          ( header::ACCESS_CONTROL_ALLOW_ORIGIN, "http://localhost:5173" ),
          ( header::ACCESS_CONTROL_ALLOW_METHODS, "POST" ),
          ( header::ACCESS_CONTROL_ALLOW_HEADERS, "Authorization,Content-Type" )
        ],
        Json(json!({ "ok": true }))
      )
    }
    _ => {
      (
        StatusCode::OK,
        [
          ( header::ACCESS_CONTROL_ALLOW_ORIGIN, "http://localhost:5173" ),
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
      ( header::ACCESS_CONTROL_ALLOW_ORIGIN, "http://localhost:5173" ),
      ( header::ACCESS_CONTROL_ALLOW_METHODS, "POST" ),
      ( header::ACCESS_CONTROL_ALLOW_HEADERS, "Authorization,Content-Type" )
    ],
    "200 OK"
  )
}