use std::sync::Arc;
use axum::{ extract::ws::Message, http::{header, HeaderMap, StatusCode}, response::IntoResponse, Extension, Json };
use serde_json::{json, Value};
use crate::{apphandler::AppHandler, util};

#[axum::debug_handler]
pub async fn put(
  Extension(app): Extension<Arc<AppHandler>>,
  headers: HeaderMap,
  Json(body): Json<Value>
) -> impl IntoResponse{
  let res = util::verify_auth::verify_auth(&headers, &app).await;
  if res.is_err(){ return res.unwrap_err(); }

  let colour = body["colour"].as_str().unwrap().to_owned();
  let id = body["id"].as_str().unwrap().to_owned();

  app.teams().set_colour(id.clone(), colour.clone()).await;
  let _ = app.live().tx.lock().await.send(Message::Text(json!({ "type": "team-colour", "team": { "_id": id, "colour": colour }, "from": res.unwrap()._id }).to_string()));

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