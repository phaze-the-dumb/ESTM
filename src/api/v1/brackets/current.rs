use std::sync::Arc;
use axum::{ http::{header, HeaderMap, StatusCode}, response::IntoResponse, Extension, Json };
use serde_json::json;
use crate::{apphandler::AppHandler, structs::config::AppState, util};

#[axum::debug_handler]
pub async fn get(
  Extension(app): Extension<Arc<AppHandler>>,
  headers: HeaderMap
) -> impl IntoResponse{
  let res = util::verify_auth::verify_auth(&headers, &app).await;
  if res.is_err(){ return res.unwrap_err(); }

  let config = app.config().get().await;

  match config.current_state{
    AppState::PLAYING => {
      let next_bracket = app.get_next_bracket(&config).await;
      let current_bracket = app.get_current_bracket(&config).await;

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
      let bracket ;

      if current_bracket.is_err(){
        bracket = ( None, None, 0 );
      } else{
        bracket = current_bracket.unwrap();
      }

      (
        StatusCode::OK,
        [
          ( header::ACCESS_CONTROL_ALLOW_ORIGIN, "http://localhost:5173" ),
          ( header::ACCESS_CONTROL_ALLOW_METHODS, "GET" ),
          ( header::ACCESS_CONTROL_ALLOW_HEADERS, "Authorization" )
        ],
        Json(json!({ "ok": true, "next": next_bracket, "current": bracket }))
      )
    }
    _ => {
      (
        StatusCode::OK,
        [
          ( header::ACCESS_CONTROL_ALLOW_ORIGIN, "http://localhost:5173" ),
          ( header::ACCESS_CONTROL_ALLOW_METHODS, "GET" ),
          ( header::ACCESS_CONTROL_ALLOW_HEADERS, "Authorization" )
        ],
        Json(json!({ "ok": false, "error": "Not in play mode" }))
      )
    }
  }
}

pub async fn options() -> impl IntoResponse{
  (
    StatusCode::OK,
    [
      ( header::ACCESS_CONTROL_ALLOW_ORIGIN, "http://localhost:5173" ),
      ( header::ACCESS_CONTROL_ALLOW_METHODS, "GET" ),
      ( header::ACCESS_CONTROL_ALLOW_HEADERS, "Authorization" )
    ],
    "200 OK"
  )
}