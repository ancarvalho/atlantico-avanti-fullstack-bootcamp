use serde::{Deserialize, Serialize};
use sqlx::{types::Uuid, FromRow};

#[derive(Clone, Debug, Deserialize, Serialize, FromRow)]
pub struct Place {
    pub id: Uuid,
    pub name: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub address: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub city: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub state: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub neighborhood: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreatePlace {
    pub name: String,
    pub address: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub country: Option<String>,
    pub neighborhood: Option<String>,
}



#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdatePlace {
    pub name: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub country: Option<String>,
    pub neighborhood: Option<String>,
}