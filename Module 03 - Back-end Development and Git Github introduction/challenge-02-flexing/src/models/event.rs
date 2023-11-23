use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize, FromRow)]
pub struct Event {
  pub id: Uuid,
  pub name: String,
  pub description: Option<String>,
  pub date: NaiveDateTime,
  pub category_id: Uuid,
  pub place_id: Uuid,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateEvent {
  pub name: String,
  pub description: Option<String>,
  pub date: NaiveDateTime,
  pub category_id: Uuid,
  pub place_id: Uuid,
}


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EventUpdate {
  pub id: Option<Uuid>,
  pub name: Option<String>,
  pub description: Option<String>,
  pub date: Option<NaiveDateTime>,
  pub category_id: Option<Uuid>,
  pub place_id: Option<Uuid>,
}
