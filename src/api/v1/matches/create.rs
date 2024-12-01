use std::sync::Arc;
use axum::{ extract::ws::Message, http::{header, HeaderMap, StatusCode}, response::IntoResponse, Extension, Json };
use serde_json::{json, Value};
use crate::{apphandler::AppHandler, util};

#[axum::debug_handler]
pub async fn post(
  Extension(app): Extension<Arc<AppHandler>>,
  headers: HeaderMap,
  Json(body): Json<Value>
) -> impl IntoResponse{
  let res = util::verify_auth::verify_auth(&headers, &app).await;
  if res.is_err(){ return res.unwrap_err(); }

  let name = body["name"].as_str().unwrap().to_owned();
  let id = app.matches().create_match(name.clone()).await;

  let _ = app.live().tx.lock().await.send(Message::Text(json!({ "type": "create-match", "match": { "_id": id, "name": name }, "from": res.unwrap()._id }).to_string()));

  (
    StatusCode::OK,
    [
      ( header::ACCESS_CONTROL_ALLOW_ORIGIN, "*" ),
      ( header::ACCESS_CONTROL_ALLOW_METHODS, "POST" ),
      ( header::ACCESS_CONTROL_ALLOW_HEADERS, "Authorization,Content-Type" )
    ],
    Json(json!({ "ok": true, "_id": id }))
  )
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