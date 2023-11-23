use std::sync::Arc;

use axum::{Router, routing::{post, get, patch, delete}, body::Body};

use crate::{controllers::place_controller::PlaceController, AppData};


pub struct PlaceRouter;

impl PlaceRouter {
  pub fn new() -> Router<Arc<AppData>, Body>{
    return Router::new()
      .route("/", post(PlaceController::create_place))
      .route("/all", get(PlaceController::get_all_places))
      .route("/unique/:place_id", get(PlaceController::get_place))
      .route("/:place_id", patch(PlaceController::update_place))
      .route("/:place_id", delete(PlaceController::delete_place))

    
  }
}