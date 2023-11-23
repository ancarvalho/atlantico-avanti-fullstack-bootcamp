use std::sync::Arc;

use axum::{Router, routing::{post, get, patch, delete}, body::Body};

use crate::{controllers::category_controller::CategoryController, AppData};

pub struct CategoryRouter;

impl CategoryRouter {
  pub fn new() -> Router<Arc<AppData>, Body>{
    return Router::new()
      .route("/", post(CategoryController::create_category))
      .route("/all", get(CategoryController::get_all_categories))
      .route("/unique/:category_id", get(CategoryController::get_category))
      .route("/:category_id", patch(CategoryController::update_category))
      .route("/:category_id", delete(CategoryController::delete_category))

    
  }
}