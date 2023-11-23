use std::sync::Arc;

use axum::{
  extract::{Path, State, self},
  http::StatusCode,
  response::IntoResponse,
  Json,
};
use uuid::Uuid;

use crate::{models::category::Category, AppData};


pub struct CategoryController;

impl CategoryController {


  pub async fn get_all_categories(State(data): State<Arc<AppData>>) -> impl IntoResponse {
    match data.category_repo.get_categories().await {
      Ok(places) => Ok(Json(places)),
      Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
  }

  pub async fn get_category(
    State(app_data): State<Arc<AppData>>,
    Path(place_id): Path<Uuid>,
  ) -> impl IntoResponse {
    match app_data.category_repo.get_category(place_id).await {
      Ok(place) => Ok(Json(place)),
      Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
  }

  pub async fn create_category(
    State(app_data): State<Arc<AppData>>,
    extract::Json(p): extract::Json<Category>
  ) -> impl IntoResponse {
    match app_data.category_repo.create_category(p).await {
      Ok(_) => Ok(StatusCode::CREATED),
      Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
  }

  pub async fn update_category(
    State(app_data): State<Arc<AppData>>,
    Path(event_id): Path<Uuid>,
    extract::Json(p): extract::Json<Category>,
  ) -> impl IntoResponse {
    match app_data.category_repo.update_category(p, event_id).await {
      Ok(_) => Ok(StatusCode::OK),
      Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
  }

  pub async fn delete_category(
    State(app_data): State<Arc<AppData>>,
    Path(place_id): Path<Uuid>
  ) -> impl IntoResponse {
    match app_data.category_repo.delete_category(place_id).await {
      Ok(_) => Ok(StatusCode::OK),
      Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
  }

}
