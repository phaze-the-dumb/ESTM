use std::sync::Arc;
use axum::{ http::{header, HeaderMap, StatusCode}, response::IntoResponse, Extension, Json };
use serde_json::json;
use crate::{apphandler::AppHandler, util};

#[axum::debug_handler]
pub async fn get(
  Extension(app): Extension<Arc<AppHandler>>,
  headers: HeaderMap
) -> impl IntoResponse{
  let res = util::verify_auth::verify_auth(&headers, &app).await;
  if res.is_err(){ return res.unwrap_err(); }

  let config = app.config().get().await;
  let game = app.matches().get(&config.current_match).await;

  if game.is_none(){
    return (
      StatusCode::OK,
      [
        ( header::ACCESS_CONTROL_ALLOW_ORIGIN, "http://localhost:5173" ),
        ( header::ACCESS_CONTROL_ALLOW_METHODS, "GET" ),
        ( header::ACCESS_CONTROL_ALLOW_HEADERS, "Authorization" )
      ],
      Json(json!({ "ok": true }))
    )
  }

  (
    StatusCode::OK,
    [
      ( header::ACCESS_CONTROL_ALLOW_ORIGIN, "http://localhost:5173" ),
      ( header::ACCESS_CONTROL_ALLOW_METHODS, "GET" ),
      ( header::ACCESS_CONTROL_ALLOW_HEADERS, "Authorization" )
    ],
    Json(json!({ "ok": true, "match": game.unwrap() }))
  )
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