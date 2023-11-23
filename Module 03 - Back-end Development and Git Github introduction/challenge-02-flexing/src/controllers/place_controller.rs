use std::sync::Arc;

use axum::{
  extract::{Path, State, self},
  http::StatusCode,
  response::IntoResponse,
  Json,
};
use uuid::Uuid;

use crate::{
  models::place::Place, AppData, 

};


pub struct PlaceController;

impl PlaceController {


  pub async fn get_all_places(State(app_data): State<Arc<AppData>>) -> impl IntoResponse {
    match app_data.place_repo.get_places().await {
      Ok(places) => Ok(Json(places)),
      Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
  }

  pub async fn get_place(
    State(app_data): State<Arc<AppData>>,
    Path(place_id): Path<Uuid>,
  ) -> impl IntoResponse {
    match app_data.place_repo.get_place(place_id).await {
      Ok(place) => Ok(Json(place)),
      Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
  }

  pub async fn create_place(
    State(app_data): State<Arc<AppData>>,
    extract::Json(p): extract::Json<Place>
  ) -> impl IntoResponse {
    match app_data.place_repo.create_place(p).await {
      Ok(_) => Ok(StatusCode::CREATED),
      Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
  }

  pub async fn update_place(
    State(app_data): State<Arc<AppData>>,
    Path(event_id): Path<Uuid>,
    extract::Json(p): extract::Json<Place>,
  ) -> impl IntoResponse {
    match app_data.place_repo.update_place(p, event_id).await {
      Ok(_) => Ok(StatusCode::OK),
      Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
  }

  pub async fn delete_place(
    State(app_data): State<Arc<AppData>>,
    Path(place_id): Path<Uuid>
  ) -> impl IntoResponse {
    match app_data.place_repo.delete_place(place_id).await {
      Ok(_) => Ok(StatusCode::OK),
      Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
  }

}
