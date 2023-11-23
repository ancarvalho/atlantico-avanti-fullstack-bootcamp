use std::sync::Arc;

use crate::repositories::{event_repository::EventRepository, place_repository::PlaceRepo, category_repository::CategoryRepo};

pub struct AppData {
  pub event_repo: EventRepository,
  pub place_repo: PlaceRepo,
  pub category_repo: CategoryRepo,
}

pub type AppDataArc = Arc<AppData>;

impl AppData {
  pub fn new(
    event_repo: EventRepository,
    place_repo: PlaceRepo,
    category_repo: CategoryRepo,
  ) -> AppDataArc {
    Arc::new(AppData {
      event_repo,
      place_repo,
      category_repo,
    })
  }
}
