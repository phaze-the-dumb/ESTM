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
    AppState::EDITING => {
      let id = body["id"].as_str().unwrap().to_owned();
      let name = body["name"].as_str().unwrap().to_owned();

      app.matches().rename(id.clone(), name.clone()).await;
      let _ = app.live().tx.lock().await.send(Message::Text(json!({ "type": "rename-match", "match": { "_id": id, "name": name }, "from": res.unwrap()._id }).to_string()));

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
      (
        StatusCode::OK,
        [
          ( header::ACCESS_CONTROL_ALLOW_ORIGIN, "*" ),
          ( header::ACCESS_CONTROL_ALLOW_METHODS, "PUT" ),
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
      ( header::ACCESS_CONTROL_ALLOW_METHODS, "PUT" ),
      ( header::ACCESS_CONTROL_ALLOW_HEADERS, "Authorization,Content-Type" )
    ],
    "200 OK"
  )
}