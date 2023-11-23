use axum::{
  extract::{self, Path, Query, State},
  http::StatusCode,
  response::IntoResponse,
  Json,
};
use uuid::Uuid;

use crate::models::{
  app_data::AppDataArc,
  event::{CreateEvent, EventUpdate},
  filters::{Filters, QueryParams},
};

pub struct EventController;

impl EventController {
  pub async fn filter_events(
    State(app_data): State<AppDataArc>,
    query_params: Query<QueryParams>,
  ) -> impl IntoResponse {
    return match Filters::parse_query_params(query_params) {
      Ok(filters) => match app_data.event_repo.get_filtered_events(filters).await {
        Ok(events) => Ok(Json(events)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
      },
      Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    };
  }

  pub async fn get_all_events(State(app_data): State<AppDataArc>) -> impl IntoResponse {
    match app_data.event_repo.get_events().await {
      Ok(events) => Ok(Json(events)),
      Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
  }

  pub async fn get_event(
    State(app_data): State<AppDataArc>,
    Path(event_id): Path<Uuid>,
  ) -> impl IntoResponse {
    match app_data.event_repo.get_event(event_id).await {
      Ok(events) => Ok(Json(events)),
      Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
  }

  pub async fn create_event(
    State(app_data): State<AppDataArc>,
    extract::Json(e): extract::Json<CreateEvent>,
  ) -> impl IntoResponse {
    match app_data.event_repo.create_event(e).await {
      Ok(events) => Ok(Json(events)),
      Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
  }

  pub async fn update_event(
    State(app_data): State<AppDataArc>,
    Path(event_id): Path<Uuid>,
    extract::Json(e): extract::Json<EventUpdate>,
  ) -> impl IntoResponse {
    match app_data.event_repo.update_event(e, event_id).await {
      Ok(_) => Ok(StatusCode::OK),
      Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
  }

  pub async fn delete_event(
    State(app_data): State<AppDataArc>,
    Path(event_id): Path<Uuid>,
  ) -> impl IntoResponse {
    match app_data.event_repo.delete_event(event_id).await {
      Ok(_) => Ok(StatusCode::OK),
      Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
  }
}
