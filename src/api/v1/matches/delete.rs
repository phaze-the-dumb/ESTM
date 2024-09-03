use std::{collections::HashMap, sync::Arc};
use axum::{ extract::{ws::Message, Query}, http::{header, HeaderMap, StatusCode}, response::IntoResponse, Extension, Json };
use bson::doc;
use serde_json::json;
use crate::{apphandler::AppHandler, util, structs::config::AppState};

#[axum::debug_handler]
pub async fn delete(
  Extension(app): Extension<Arc<AppHandler>>,
  headers: HeaderMap,
  Query(query): Query<HashMap<String, String>>
) -> impl IntoResponse{
  let res = util::verify_auth::verify_auth(&headers, &app).await;
  if res.is_err(){ return res.unwrap_err(); }

  let config = app.config().get().await;
  match config.current_state{
    AppState::EDITING => {
      let id = query.get("id").unwrap().clone();

      app.matches().delete(id.clone()).await;
      let _ = app.live().tx.lock().await.send(Message::Text(json!({ "type": "delete-match", "match": { "_id": id }, "from": res.unwrap()._id }).to_string()));

      app.teams().delete_in_match(id.clone()).await;
      app.brackets().delete_in_match(id.clone()).await;

      (
        StatusCode::OK,
        [
          ( header::ACCESS_CONTROL_ALLOW_ORIGIN, "http://localhost:5173" ),
          ( header::ACCESS_CONTROL_ALLOW_METHODS, "DELETE" ),
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
          ( header::ACCESS_CONTROL_ALLOW_METHODS, "DELETE" ),
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
      ( header::ACCESS_CONTROL_ALLOW_METHODS, "DELETE" ),
      ( header::ACCESS_CONTROL_ALLOW_HEADERS, "Authorization,Content-Type" )
    ],
    "200 OK"
  )
}