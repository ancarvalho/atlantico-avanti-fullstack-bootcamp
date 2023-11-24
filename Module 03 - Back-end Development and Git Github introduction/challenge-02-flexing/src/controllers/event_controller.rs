use axum::{
  extract::{self, Path, Query, State},
  http::StatusCode,
  response::IntoResponse,
  Json,
};
use uuid::Uuid;

use crate::models::{
  app_data::AppDataArc,
  event::{CreateEvent, EventUpdate, Event},
  filters::{Filters, QueryParams}, status,
};

pub struct EventController;

impl EventController {
  pub async fn filter_events(
    State(app_data): State<AppDataArc>,
    query_params: Query<QueryParams>,
  ) -> impl IntoResponse {
    return match Filters::parse_query_params(query_params) {
      Ok(filters) => match app_data.event_repo.get_filtered_events(filters).await {
        Ok(events) => Ok((
          StatusCode::OK,
          Json(status::Response::<Vec<Event>>::new_response(
            "Events Listed Successfully",
            Some(events),
          )),
        )),
        Err(e) => Err((
          StatusCode::INTERNAL_SERVER_ERROR,
          Json(status::Response::<i8>::new_error_response(
            "Error Parsing Query",
            Some(e.to_string()),
          )),
        )),
      },
      Err(e) => Err((
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(status::Response::<i8>::new_error_response(
          "Error Filtering Event",
          Some(e.to_string()),
        )),
      )),
    };
  }

  pub async fn get_all_events(State(app_data): State<AppDataArc>) -> impl IntoResponse {
    match app_data.event_repo.get_events().await {
      Ok(events) => Ok((
        StatusCode::OK,
        Json(status::Response::<Vec<Event>>::new_response(
          "Events Listed Successfully",
          Some(events),
        )),
      )),
      Err(e) => Err((
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(status::Response::<i8>::new_error_response(
          "Error Listing Event",
          Some(e.to_string()),
        )),
      )),
    }
  }

  pub async fn get_event(
    State(app_data): State<AppDataArc>,
    Path(event_id): Path<Uuid>,
  ) -> impl IntoResponse {
    match app_data.event_repo.get_event(event_id).await {
      // Ok(events) => Ok(Json(events)),
      Ok(event) => Ok((
        StatusCode::OK,
        Json(status::Response::<Event>::new_response(
          "Event Listed Successfully",
          Some(event),
        )),
      )),
      Err(e) => Err((
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(status::Response::<i8>::new_error_response(
          "Error Retrieving Event",
          Some(e.to_string()),
        )),
      )),
    }
  }

  pub async fn create_event(
    State(app_data): State<AppDataArc>,
    extract::Json(e): extract::Json<CreateEvent>,
  ) -> impl IntoResponse {
    match app_data.event_repo.create_event(e).await {
      Ok(_) => Ok((
        StatusCode::CREATED,
        Json(status::Response::<i8>::new_response(
          "Event Created Successfully",
          None,
        )),
      )),
      Err(e) => Err((
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(status::Response::<i8>::new_error_response(
          "Error Creating Event",
          Some(e.to_string()),
        )),
      )),
    }
  }

  pub async fn update_event(
    State(app_data): State<AppDataArc>,
    Path(event_id): Path<Uuid>,
    extract::Json(e): extract::Json<EventUpdate>,
  ) -> impl IntoResponse {
    match app_data.event_repo.update_event(e, event_id).await {
      Ok(_) => Ok((
        StatusCode::OK,
        Json(status::Response::<i8>::new_response(
          "Event Updated Successfully",
          None,
        )),
      )),
      Err(e) => Err((
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(status::Response::<i8>::new_error_response(
          "Error Updating Event",
          Some(e.to_string()),
        )),
      )),
    }
  }

  pub async fn delete_event(
    State(app_data): State<AppDataArc>,
    Path(event_id): Path<Uuid>,
  ) -> impl IntoResponse {
    match app_data.event_repo.delete_event(event_id).await {
      Ok(_) => Ok((
        StatusCode::OK,
        Json(status::Response::<i8>::new_response(
          "Event Deleted Successfully",
          None,
        )),
      )),
      Err(e) => Err((
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(status::Response::<i8>::new_error_response(
          "Error Deleting Event",
          Some(e.to_string()),
        )),
      )),
    }
  }
}
// Ok(_) => Ok((
//   StatusCode::OK,
//   Json(status::Response::<i8>::new_response(
//     "Category Deleted Successfully",
//     None,
//   )),
// )),
// Err(e) => Err((
//   StatusCode::INTERNAL_SERVER_ERROR,
//   Json(status::Response::<i8>::new_error_response(
//     "Error Deleting Category",
//     Some(e.to_string()),
//   )),
// )),