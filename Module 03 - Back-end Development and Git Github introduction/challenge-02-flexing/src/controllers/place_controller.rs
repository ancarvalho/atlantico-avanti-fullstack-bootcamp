use axum::{
  extract::{self, Path, State},
  http::StatusCode,
  response::IntoResponse,
  Json,
};
use uuid::Uuid;

use crate::models::{
  app_data::AppDataArc,
  place::{CreatePlace, Place, UpdatePlace},
  status,
};

pub struct PlaceController;

impl PlaceController {
  pub async fn get_all_places(State(app_data): State<AppDataArc>) -> impl IntoResponse {
    match app_data.place_repo.get_places().await {
      Ok(place) => Ok((
        StatusCode::OK,
        Json(status::Response::<Vec<Place>>::new_response(
          "Places Listed Successfully",
          Some(place),
        )),
      )),
      Err(e) => Err((
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(status::Response::<i8>::new_error_response(
          "Error Listing Places",
          Some(e.to_string()),
        )),
      )),
    }
  }

  pub async fn get_place(
    State(app_data): State<AppDataArc>,
    Path(place_id): Path<Uuid>,
  ) -> impl IntoResponse {
    match app_data.place_repo.get_place(place_id).await {
      Ok(place) => Ok((
        StatusCode::OK,
        Json(status::Response::<Place>::new_response(
          "Place Listed Successfully",
          Some(place),
        )),
      )),
      Err(e) => Err((
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(status::Response::<i8>::new_error_response(
          "Error Listing Place",
          Some(e.to_string()),
        )),
      )),
    }
  }

  pub async fn create_place(
    State(app_data): State<AppDataArc>,
    extract::Json(p): extract::Json<CreatePlace>,
  ) -> impl IntoResponse {
    match app_data.place_repo.create_place(p).await {
      Ok(_) => Ok((
        StatusCode::CREATED,
        Json(status::Response::<i8>::new_response(
          "Place Created Successfully",
          None,
        )),
      )),
      Err(e) => Err((
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(status::Response::<i8>::new_error_response(
          "Error Creating Place",
          Some(e.to_string()),
        )),
      )),
    }
  }

  pub async fn update_place(
    State(app_data): State<AppDataArc>,
    Path(event_id): Path<Uuid>,
    extract::Json(p): extract::Json<UpdatePlace>,
  ) -> impl IntoResponse {
    match app_data.place_repo.update_place(p, event_id).await {
      Ok(_) => Ok((
        StatusCode::OK,
        Json(status::Response::<i8>::new_response(
          "Place Updated Successfully",
          None,
        )),
      )),
      Err(e) => Err((
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(status::Response::<i8>::new_error_response(
          "Error Updating Place",
          Some(e.to_string()),
        )),
      )),
    }
  }

  pub async fn delete_place(
    State(app_data): State<AppDataArc>,
    Path(place_id): Path<Uuid>,
  ) -> impl IntoResponse {
    match app_data.place_repo.delete_place(place_id).await {
      Ok(_) => Ok((
        StatusCode::OK,
        Json(status::Response::<i8>::new_response(
          "Place Deleted Successfully",
          None,
        )),
      )),
      Err(e) => Err((
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(status::Response::<i8>::new_error_response(
          "Error Deleting Place",
          Some(e.to_string()),
        )),
      )),
    }
  }
}
