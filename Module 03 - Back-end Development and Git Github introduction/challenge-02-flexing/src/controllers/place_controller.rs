use axum::{
  extract::{self, Path, State},
  http::StatusCode,
  response::IntoResponse,
  Json,
};
use uuid::Uuid;

use crate::models::{
  app_data::AppDataArc,
  place::{CreatePlace, UpdatePlace},
};

pub struct PlaceController;

impl PlaceController {
  pub async fn get_all_places(State(app_data): State<AppDataArc>) -> impl IntoResponse {
    match app_data.place_repo.get_places().await {
      Ok(places) => Ok(Json(places)),
      Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
  }

  pub async fn get_place(
    State(app_data): State<AppDataArc>,
    Path(place_id): Path<Uuid>,
  ) -> impl IntoResponse {
    match app_data.place_repo.get_place(place_id).await {
      Ok(place) => Ok(Json(place)),
      Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
  }

  pub async fn create_place(
    State(app_data): State<AppDataArc>,
    extract::Json(p): extract::Json<CreatePlace>,
  ) -> impl IntoResponse {
    match app_data.place_repo.create_place(p).await {
      Ok(_) => Ok(StatusCode::CREATED),
      Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
  }

  pub async fn update_place(
    State(app_data): State<AppDataArc>,
    Path(event_id): Path<Uuid>,
    extract::Json(p): extract::Json<UpdatePlace>,
  ) -> impl IntoResponse {
    match app_data.place_repo.update_place(p, event_id).await {
      Ok(_) => Ok(StatusCode::OK),
      Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
  }

  pub async fn delete_place(
    State(app_data): State<AppDataArc>,
    Path(place_id): Path<Uuid>,
  ) -> impl IntoResponse {
    match app_data.place_repo.delete_place(place_id).await {
      Ok(_) => Ok(StatusCode::OK),
      Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
  }
}
