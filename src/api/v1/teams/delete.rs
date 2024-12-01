use std::{collections::HashMap, sync::Arc};
use axum::{ extract::{ws::Message, Query}, http::{header, HeaderMap, StatusCode}, response::IntoResponse, Extension, Json };
use serde_json::json;
use crate::{apphandler::AppHandler, util};

#[axum::debug_handler]
pub async fn delete(
  Extension(app): Extension<Arc<AppHandler>>,
  headers: HeaderMap,
  Query(query): Query<HashMap<String, String>>
) -> impl IntoResponse{
  let res = util::verify_auth::verify_auth(&headers, &app).await;
  if res.is_err(){ return res.unwrap_err(); }

  let id = query.get("id").unwrap();
  let match_id = app.teams().delete(id.clone()).await;

  let _ = app.live().tx.lock().await.send(Message::Text(json!({ "type": "delete-team", "team": { "_id": id }, "from": res.unwrap()._id }).to_string()));
  app.brackets().generate(app.teams().count(match_id.clone()).await, match_id).await;

  (
    StatusCode::OK,
    [
      ( header::ACCESS_CONTROL_ALLOW_ORIGIN, "*" ),
      ( header::ACCESS_CONTROL_ALLOW_METHODS, "DELETE" ),
      ( header::ACCESS_CONTROL_ALLOW_HEADERS, "Authorization" )
    ],
    Json(json!({ "ok": true }))
  )
}

pub async fn options() -> impl IntoResponse{
  (
    StatusCode::OK,
    [
      ( header::ACCESS_CONTROL_ALLOW_ORIGIN, "*" ),
      ( header::ACCESS_CONTROL_ALLOW_METHODS, "DELETE" ),
      ( header::ACCESS_CONTROL_ALLOW_HEADERS, "Authorization" )
    ],
    "200 OK"
  )
}