use std::sync::Arc;

use axum::{http::{header, HeaderMap, HeaderName, StatusCode}, Json};
use serde_json::{json, Value};

use crate::{apphandler::AppHandler, structs::session::Session};

pub async fn verify_auth( headers: &HeaderMap, app: &Arc<AppHandler> ) -> Result<Session, (StatusCode, [(HeaderName, &'static str); 3], Json<Value>)>{
  let auth = headers.get("Authorization");

  if auth.is_none(){
    return Err((
      StatusCode::UNAUTHORIZED,
      [
        ( header::ACCESS_CONTROL_ALLOW_ORIGIN, "http://localhost:5173" ),
        ( header::ACCESS_CONTROL_ALLOW_METHODS, "GET" ),
        ( header::ACCESS_CONTROL_ALLOW_HEADERS, "Authorization" )
      ],
      Json(json!({ "ok": false, "error": "Invalid token" }))
    ))
  }

  let auth = auth.unwrap().to_str().unwrap().replace("Bearer ", "");
  app.auth().clean_sessions().await;

  let session= app.auth().verify_token(auth).await;

  if session.is_none(){
    return Err((
      StatusCode::UNAUTHORIZED,
      [
        ( header::ACCESS_CONTROL_ALLOW_ORIGIN, "http://localhost:5173" ),
        ( header::ACCESS_CONTROL_ALLOW_METHODS, "GET" ),
        ( header::ACCESS_CONTROL_ALLOW_HEADERS, "Authorization" )
      ],
      Json(json!({ "ok": false, "error": "Invalid token" }))
    ))
  } else{
    Ok(session.unwrap())
  }
}