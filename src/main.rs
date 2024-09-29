mod apphandler;
mod api;
mod structs;
mod util;
mod db_controller;

use std::sync::Arc;
use axum::{ routing::{ delete, get, options, post, put }, Extension, Router };
use dotenvy::dotenv;
use apphandler::AppHandler;

#[tokio::main]
async fn main() {
  dotenv().expect(".env file not found"); // Load the ENV file into environment variables
  let handle = AppHandler::new().await; //

  handle.auth().clean_sessions().await;
  start_webserver(handle);

  loop{}
}

fn start_webserver( handle: Arc<AppHandler> ){
  tokio::spawn(async move {
    let app = Router::new()
      .route("/api/v1/live", get(api::v1::live::get))

      // Auth Routes
      .route("/api/v1/auth/login", options(api::v1::auth::login::options))
      .route("/api/v1/auth/login", post(api::v1::auth::login::post))

      .route("/api/v1/auth/verify", options(api::v1::auth::verify::options))
      .route("/api/v1/auth/verify", get(api::v1::auth::verify::get))

      // Matches Routes
      .route("/api/v1/matches/start", options(api::v1::matches::start::options))
      .route("/api/v1/matches/start", post(api::v1::matches::start::post))

      .route("/api/v1/matches/cancel", options(api::v1::matches::cancel::options))
      .route("/api/v1/matches/cancel", post(api::v1::matches::cancel::post))

      .route("/api/v1/matches/next", options(api::v1::matches::next::options))
      .route("/api/v1/matches/next", post(api::v1::matches::next::post))

      .route("/api/v1/matches/create", options(api::v1::matches::create::options))
      .route("/api/v1/matches/create", post(api::v1::matches::create::post))

      .route("/api/v1/matches/select", options(api::v1::matches::select::options))
      .route("/api/v1/matches/select", put(api::v1::matches::select::put))

      .route("/api/v1/matches/list", options(api::v1::matches::list::options))
      .route("/api/v1/matches/list", get(api::v1::matches::list::get))

      .route("/api/v1/matches/get", options(api::v1::matches::get::options))
      .route("/api/v1/matches/get", get(api::v1::matches::get::get))

      .route("/api/v1/matches/selected", options(api::v1::matches::selected::options))
      .route("/api/v1/matches/selected", get(api::v1::matches::selected::get))

      .route("/api/v1/matches/rename", options(api::v1::matches::rename::options))
      .route("/api/v1/matches/rename", put(api::v1::matches::rename::put))

      .route("/api/v1/matches/delete", options(api::v1::matches::delete::options))
      .route("/api/v1/matches/delete", delete(api::v1::matches::delete::delete))

      // Teams Routes
      .route("/api/v1/teams/create", options(api::v1::teams::create::options))
      .route("/api/v1/teams/create", post(api::v1::teams::create::post))

      .route("/api/v1/teams/list", options(api::v1::teams::list::options))
      .route("/api/v1/teams/list", get(api::v1::teams::list::get))

      .route("/api/v1/teams/rename", options(api::v1::teams::rename::options))
      .route("/api/v1/teams/rename", put(api::v1::teams::rename::put))

      .route("/api/v1/teams/delete", options(api::v1::teams::delete::options))
      .route("/api/v1/teams/delete", delete(api::v1::teams::delete::delete))

      // Brackets Routes
      .route("/api/v1/brackets/get_match", options(api::v1::brackets::get_match::options))
      .route("/api/v1/brackets/get_match", get(api::v1::brackets::get_match::get))

      .route("/api/v1/brackets/current", options(api::v1::brackets::current::options))
      .route("/api/v1/brackets/current", get(api::v1::brackets::current::get))

      .route("/api/v1/brackets/winner", options(api::v1::brackets::winner::options))
      .route("/api/v1/brackets/winner", put(api::v1::brackets::winner::put))

      .layer(Extension(handle));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
  });
}