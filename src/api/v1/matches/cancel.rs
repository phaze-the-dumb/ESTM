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
  let res = util::verify_auth::verify_auth(&headers, &app).await;
  if res.is_err(){ return res.unwrap_err(); }

  let config = app.config().get().await;
  match config.current_state{
    AppState::PLAYING => {
      app.config().update(doc! { "current_state": "EDITING" }).await;
      app.brackets().reset_all(config.current_match).await;

      let _ = app.live().tx.lock().await.send(Message::Text(json!({ "type": "cancel-match", "from": res.unwrap()._id }).to_string()));

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