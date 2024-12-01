use std::{collections::HashMap, sync::Arc};
use axum::{ extract::Query, http::{header, HeaderMap, StatusCode}, response::IntoResponse, Extension, Json };
use serde_json::json;
use crate::{apphandler::AppHandler, util};

#[axum::debug_handler]
pub async fn get(
  Extension(app): Extension<Arc<AppHandler>>,
  headers: HeaderMap,
  Query(query): Query<HashMap<String, String>>
) -> impl IntoResponse{
  let res = util::verify_auth::verify_auth(&headers, &app).await;
  if res.is_err(){ return res.unwrap_err(); }

  let match_id = query.get("match_id").unwrap().clone();
  let teams = app.teams().list_teams_in_match(match_id).await;

  (
    StatusCode::OK,
    [
      ( header::ACCESS_CONTROL_ALLOW_ORIGIN, "*" ),
      ( header::ACCESS_CONTROL_ALLOW_METHODS, "POST" ),
      ( header::ACCESS_CONTROL_ALLOW_HEADERS, "Authorization,Content-Type" )
    ],
    Json(json!({ "ok": true, "teams": teams }))
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