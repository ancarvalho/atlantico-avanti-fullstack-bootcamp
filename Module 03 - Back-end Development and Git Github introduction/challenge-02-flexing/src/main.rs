mod controllers;
mod database;
mod error;
mod models;
mod repositories;
mod routes;
use std::{env, net::SocketAddr, sync::Arc};

use axum::Router;

pub struct AppData {
  event_repo: EventRepository,
  place_repo: PlaceRepo,
  category_repo: CategoryRepo,
}

use database::connect;
use repositories::{event_repository::EventRepository, category_repository::CategoryRepo, place_repository::PlaceRepo};
use routes::{event_router::EventRouter, category_route::CategoryRouter, place_routes::PlaceRouter};

#[tokio::main]
async fn main() {
  let port = env::var("PORT")
    .ok()
    .and_then(|port| port.parse::<u16>().ok())
    .unwrap_or(42069);

  let pool = connect().await.unwrap();

  let event_repo =EventRepository::new(pool.clone());
  let category_repo =CategoryRepo::new(pool.clone());
  let place_repo =PlaceRepo::new(pool);

  let app_data = Arc::new( AppData{category_repo, event_repo, place_repo});

  let app = Router::new()
    // .route(
    //   "/",
    //   get(|_req: Request<Body>| async {
    //     let res = Response::new(Body::from("Hi from `GET /`"));
    //     Ok::<_, Infallible>(res)
    //   }),
    // )
    .nest("/event", EventRouter::new())
    .nest("/category", CategoryRouter::new())
    .nest("/place", PlaceRouter::new())
    // .with_state(place_repo)
    // .with_state(category_repo)
    .with_state(app_data);

  axum::Server::bind(&SocketAddr::from(([0, 0, 0, 0], port)))
    .serve(app.into_make_service())
    .await
    .unwrap();
}
