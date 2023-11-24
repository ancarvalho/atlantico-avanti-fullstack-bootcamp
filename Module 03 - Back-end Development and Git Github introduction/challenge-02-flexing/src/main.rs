mod controllers;
mod database;
mod models;
mod repositories;
mod routes;
use std::{env, net::SocketAddr};

use axum::Router;



use database::connect;
use models::app_data::AppData;
use repositories::{
  category_repository::CategoryRepo, event_repository::EventRepository, place_repository::PlaceRepo,
};
use routes::{
  category_route::CategoryRouter, event_router::EventRouter, place_routes::PlaceRouter,
};

#[tokio::main]
async fn main() {
  let port = env::var("PORT")
    .ok()
    .and_then(|port| port.parse::<u16>().ok())
    .unwrap_or(42069);

  let pool = connect().await.unwrap();

  let event_repo = EventRepository::new(pool.clone());
  let category_repo = CategoryRepo::new(pool.clone());
  let place_repo = PlaceRepo::new(pool);



  let app = Router::new()
    .nest("/event", EventRouter::new())
    .nest("/category", CategoryRouter::new())
    .nest("/place", PlaceRouter::new())
    .with_state(AppData::new(event_repo, place_repo, category_repo));

  axum::Server::bind(&SocketAddr::from(([0, 0, 0, 0], port)))
    .serve(app.into_make_service())
    .await
    .unwrap();
}
