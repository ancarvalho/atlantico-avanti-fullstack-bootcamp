use axum::{Router, routing::{post, get, delete, put}, body::Body};

use crate::{controllers::category_controller::CategoryController, models::app_data::AppDataArc};

pub struct CategoryRouter;

impl CategoryRouter {
  pub fn new() -> Router<AppDataArc, Body>{
    return Router::new()
      .route("/", post(CategoryController::create_category))
      .route("/all", get(CategoryController::get_all_categories))
      .route("/unique/:category_id", get(CategoryController::get_category))
      .route("/:category_id", put(CategoryController::update_category))
      .route("/:category_id", delete(CategoryController::delete_category))

    
  }
}