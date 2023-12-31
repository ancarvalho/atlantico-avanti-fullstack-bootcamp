use axum::{Router, routing::{post, get, patch, delete}, body::Body};

use crate::{controllers::event_controller::EventController,  models::app_data::AppDataArc};

pub struct EventRouter;

impl EventRouter {
  pub fn new() -> Router<AppDataArc, Body>{
    return Router::new()
      .route("/", post(EventController::create_event))
      .route("/all", get(EventController::get_all_events))
      .route("/unique/:event_id", get(EventController::get_event))
      .route("/:event_id", patch(EventController::update_event))
      .route("/:event_id", delete(EventController::delete_event))
      .route("/find", get(EventController::filter_events));
    
  }
}
