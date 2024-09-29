mod apphandler;
mod api;
mod structs;
mod util;
mod db_controller;

use std::sync::Arc;
use axum::{ routing::{ delete, get, options, post, put }, Extension, Router };
use dotenvy::dotenv;
use apphandler::AppHandler;
use tower_http::services::{ServeDir, ServeFile};

#[tokio::main]
async fn main() {
  dotenv().expect(".env file not found"); // Load .env file
  let handle = AppHandler::new().await; // Create a new app handler instance

  handle.auth().clean_sessions().await; // Clean old authentication sessions from the database
  start_webserver(handle).await; // Call the start webserver function
}

async fn start_webserver( handle: Arc<AppHandler> ){
  let app = Router::new()
    // All routes below that use the "options" method are purely for
    // sending the correct headers to the client

    .route("/api/v1/live", get(api::v1::live::get)) // The websocket connection to the backend for sending data live

    // Auth Routes
    .route("/api/v1/auth/login", options(api::v1::auth::login::options))
    .route("/api/v1/auth/login", post(api::v1::auth::login::post)) // Logs the user in with the inputted 6 character code

    .route("/api/v1/auth/verify", options(api::v1::auth::verify::options))
    .route("/api/v1/auth/verify", get(api::v1::auth::verify::get)) // Verify if a token is valid

    // Matches Routes
    .route("/api/v1/matches/start", options(api::v1::matches::start::options))
    .route("/api/v1/matches/start", post(api::v1::matches::start::post)) // Start a match

    .route("/api/v1/matches/cancel", options(api::v1::matches::cancel::options))
    .route("/api/v1/matches/cancel", post(api::v1::matches::cancel::post)) // Cancel a match

    .route("/api/v1/matches/next", options(api::v1::matches::next::options))
    .route("/api/v1/matches/next", post(api::v1::matches::next::post)) // Go to the next set of brackets

    .route("/api/v1/matches/create", options(api::v1::matches::create::options))
    .route("/api/v1/matches/create", post(api::v1::matches::create::post)) // Create a new match

    .route("/api/v1/matches/select", options(api::v1::matches::select::options))
    .route("/api/v1/matches/select", put(api::v1::matches::select::put)) // Select a match

    .route("/api/v1/matches/list", options(api::v1::matches::list::options))
    .route("/api/v1/matches/list", get(api::v1::matches::list::get)) // List all matches

    .route("/api/v1/matches/get", options(api::v1::matches::get::options))
    .route("/api/v1/matches/get", get(api::v1::matches::get::get)) // Get a specific match

    .route("/api/v1/matches/selected", options(api::v1::matches::selected::options))
    .route("/api/v1/matches/selected", get(api::v1::matches::selected::get)) // Get the currently selected match

    .route("/api/v1/matches/rename", options(api::v1::matches::rename::options))
    .route("/api/v1/matches/rename", put(api::v1::matches::rename::put)) // Rename a match

    .route("/api/v1/matches/delete", options(api::v1::matches::delete::options))
    .route("/api/v1/matches/delete", delete(api::v1::matches::delete::delete)) // Delete a match

    // Teams Routes
    .route("/api/v1/teams/create", options(api::v1::teams::create::options))
    .route("/api/v1/teams/create", post(api::v1::teams::create::post)) // Create a team

    .route("/api/v1/teams/list", options(api::v1::teams::list::options))
    .route("/api/v1/teams/list", get(api::v1::teams::list::get)) // Get all teams for a match

    .route("/api/v1/teams/rename", options(api::v1::teams::rename::options))
    .route("/api/v1/teams/rename", put(api::v1::teams::rename::put)) // Rename a team

    .route("/api/v1/teams/delete", options(api::v1::teams::delete::options))
    .route("/api/v1/teams/delete", delete(api::v1::teams::delete::delete)) // Delete a team

    // Brackets Routes
    .route("/api/v1/brackets/get_match", options(api::v1::brackets::get_match::options))
    .route("/api/v1/brackets/get_match", get(api::v1::brackets::get_match::get)) // Get brackets for a specific match

    .route("/api/v1/brackets/current", options(api::v1::brackets::current::options))
    .route("/api/v1/brackets/current", get(api::v1::brackets::current::get)) // Get current bracket

    .route("/api/v1/brackets/winner", options(api::v1::brackets::winner::options))
    .route("/api/v1/brackets/winner", put(api::v1::brackets::winner::put)) // Set the winner of the current bracket

    // Listen for static pages ( e.g. main html file, styles css, and the js frontend code ).
    // We're using the "ServeDir" utility function in tower-http to do all the static file handling for us
    // We're also going to set the 404 page to the main index.html page as it has it's own router built into it

    .nest_service("/", ServeDir::new("frontend/dist").not_found_service(ServeFile::new("frontend/dist/index.html")))

    .layer(Extension(handle)); // Embed the "AppHandler" struct into axum so all routes have access to it

  let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap(); // Create a TCP listener on port 8080
  axum::serve(listener, app).await.unwrap(); // Tell axum to use this listener
}