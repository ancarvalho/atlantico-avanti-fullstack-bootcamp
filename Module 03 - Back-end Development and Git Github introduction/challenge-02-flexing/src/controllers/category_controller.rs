use axum::{
  extract::{self, Path, State},
  http::StatusCode,
  response::IntoResponse,
  Json,
};
use uuid::Uuid;

use crate::models::{
  app_data::AppDataArc,
  category::{CreateCategory, UpdateCategory, Category},
  status,
};

pub struct CategoryController;

impl CategoryController {
  pub async fn get_all_categories(State(data): State<AppDataArc>) -> impl IntoResponse {
    match data.category_repo.get_categories().await {
      Ok(categories) => Ok((
        StatusCode::OK,
        Json(status::Response::<Vec<Category>>::new_response(
          "Categories Listed Successfully",
          Some(categories),
        )),
      )),
      Err(e) => Err((
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(status::Response::<i8>::new_error_response(
          "Error Listing Categories",
          Some(e.to_string()),
        )),
      )),
    }
  }

  pub async fn get_category(
    State(app_data): State<AppDataArc>,
    Path(place_id): Path<Uuid>,
  ) -> impl IntoResponse {
    match app_data.category_repo.get_category(place_id).await {
      Ok(category) => Ok((
        StatusCode::OK,
        Json(status::Response::<Category>::new_response(
          "Category Listed Successfully",
          Some(category),
        )),
      )),
      Err(e) => Err((
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(status::Response::<i8>::new_error_response(
          "Error Retrieving Category",
          Some(e.to_string()),
        )),
      )),
    }
  }

  pub async fn create_category(
    State(app_data): State<AppDataArc>,
    extract::Json(c): extract::Json<CreateCategory>,
  ) -> impl IntoResponse {
    match app_data.category_repo.create_category(c).await {
      Ok(_) => Ok((
        StatusCode::CREATED,
        Json(status::Response::<i8>::new_response(
          "Category Created Successfully",
          None,
        )),
      )),
      Err(e) => Err((
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(status::Response::<i8>::new_error_response(
          "Error Creating Category",
          Some(e.to_string()),
        )),
      )),
    }
  }

  pub async fn update_category(
    State(app_data): State<AppDataArc>,
    Path(category_id): Path<Uuid>,
    extract::Json(p): extract::Json<UpdateCategory>,
  ) -> impl IntoResponse {
    match app_data.category_repo.update_category(p, category_id).await {
      Ok(_) => Ok((
        StatusCode::OK,
        Json(status::Response::<i8>::new_response(
          "Category Updated Successfully",
          None,
        )),
      )),
      Err(e) => Err((
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(status::Response::<i8>::new_error_response(
          "Error Updating Category",
          Some(e.to_string()),
        )),
      )),
    }
  }

  pub async fn delete_category(
    State(app_data): State<AppDataArc>,
    Path(place_id): Path<Uuid>,
  ) -> impl IntoResponse {
    match app_data.category_repo.delete_category(place_id).await {
      Ok(_) => Ok((
        StatusCode::OK,
        Json(status::Response::<i8>::new_response(
          "Category Deleted Successfully",
          None,
        )),
      )),
      Err(e) => Err((
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(status::Response::<i8>::new_error_response(
          "Error Deleting Category",
          Some(e.to_string()),
        )),
      )),
    }
  }
}
