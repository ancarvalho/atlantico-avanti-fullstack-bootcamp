use sqlx::types::Uuid;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Clone, Debug, Deserialize, Serialize, FromRow)]
pub struct Category {
    pub id: Uuid,
    pub name: String,
}
