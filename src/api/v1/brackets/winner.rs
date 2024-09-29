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
  let res = util::verify_auth::verify_auth(&headers, &app).await;
  if res.is_err(){ return res.unwrap_err(); }

  let config = app.config().get().await;
  match config.current_state{
    AppState::PLAYING => {
      let team = body["team"].as_str().unwrap();
      let current_bracket = app.brackets().get_bracket(config.current_bracket_set, config.current_bracket, config.current_match.clone()).await.unwrap();

      match team{
        "team1" => {
          let winner = app.brackets().get_bracket(config.current_bracket_set - 1, current_bracket.team1 as u32, config.current_match.clone()).await.unwrap().winner;
          app.brackets().set_winner(config.current_bracket_set, config.current_bracket, config.current_match.clone(), winner as u32).await;
        }
        "team2" => {
          let winner = app.brackets().get_bracket(config.current_bracket_set - 1, current_bracket.team2 as u32, config.current_match.clone()).await.unwrap().winner;
          app.brackets().set_winner(config.current_bracket_set, config.current_bracket, config.current_match.clone(), winner as u32).await;
        }
        _ => {
          return (
            StatusCode::OK,
            [
              ( header::ACCESS_CONTROL_ALLOW_ORIGIN, "http://localhost:5173" ),
              ( header::ACCESS_CONTROL_ALLOW_METHODS, "PUT" ),
              ( header::ACCESS_CONTROL_ALLOW_HEADERS, "Authorization,Content-Type" )
            ],
            Json(json!({ "ok": false, "error": "Invalid team value" }))
          )
        }
      }

      let _ = app.live().tx.lock().await.send(Message::Text(json!({ "type": "win-bracket", "team": team, "from": res.unwrap()._id }).to_string()));
      let next_bracket = app.get_next_bracket(config).await;

      (
        StatusCode::OK,
        [
          ( header::ACCESS_CONTROL_ALLOW_ORIGIN, "http://localhost:5173" ),
          ( header::ACCESS_CONTROL_ALLOW_METHODS, "PUT" ),
          ( header::ACCESS_CONTROL_ALLOW_HEADERS, "Authorization,Content-Type" )
        ],
        Json(json!({ "ok": true, "next_bracket": next_bracket }))
      )
    }
    _ => {
      (
        StatusCode::OK,
        [
          ( header::ACCESS_CONTROL_ALLOW_ORIGIN, "http://localhost:5173" ),
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
      ( header::ACCESS_CONTROL_ALLOW_ORIGIN, "http://localhost:5173" ),
      ( header::ACCESS_CONTROL_ALLOW_METHODS, "PUT" ),
      ( header::ACCESS_CONTROL_ALLOW_HEADERS, "Authorization,Content-Type" )
    ],
    "200 OK"
  )
}