use std::sync::Arc;
use axum::{ http::{header, StatusCode}, response::IntoResponse, Extension, Json };
use serde_json::{json, Value};
use crate::apphandler::AppHandler;

pub async fn post(
  Extension(app): Extension<Arc<AppHandler>>,
  Json(body): Json<Value>
) -> impl IntoResponse{
  if !body["code"].is_string(){
    return (
      StatusCode::BAD_REQUEST,
      [
        ( header::ACCESS_CONTROL_ALLOW_ORIGIN, "*" ),
        ( header::ACCESS_CONTROL_ALLOW_METHODS, "POST" ),
        ( header::ACCESS_CONTROL_ALLOW_HEADERS, "Content-Type" )
      ],
      Json(json!({ "ok": false, "error": "Cannot validate request body" }))
    )
  }

  let code = body["code"].as_str().unwrap().to_owned();
  if !app.verify_code(code){
    return (
      StatusCode::UNAUTHORIZED,
      [
        ( header::ACCESS_CONTROL_ALLOW_ORIGIN, "*" ),
        ( header::ACCESS_CONTROL_ALLOW_METHODS, "POST" ),
        ( header::ACCESS_CONTROL_ALLOW_HEADERS, "Content-Type" )
      ],
      Json(json!({ "ok": false, "error": "Incorrect code" }))
    )
  }

  let ( token, id ) = app.auth().new_session().await;

  (
    StatusCode::OK,
    [
      ( header::ACCESS_CONTROL_ALLOW_ORIGIN, "*" ),
      ( header::ACCESS_CONTROL_ALLOW_METHODS, "POST" ),
      ( header::ACCESS_CONTROL_ALLOW_HEADERS, "Content-Type" )
    ],
    Json(json!({ "ok": true, "token": token, "id": id }))
  )
}

pub async fn options() -> impl IntoResponse{
  (
    StatusCode::OK,
    [
      ( header::ACCESS_CONTROL_ALLOW_ORIGIN, "*" ),
      ( header::ACCESS_CONTROL_ALLOW_METHODS, "POST" ),
      ( header::ACCESS_CONTROL_ALLOW_HEADERS, "Content-Type" )
    ],
    "200 OK"
  )
}