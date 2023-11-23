

use sqlx::{Pool, Postgres};

use anyhow::{Ok, Result};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::place::{Place, UpdatePlace, CreatePlace};




pub struct PlaceRepo {
  pub pool: Pool<Postgres>,
}

impl PlaceRepo {
  pub fn new(pool: PgPool) -> Self {
    Self { pool }
  }

  pub async fn get_place(&self, id: Uuid) -> Result<Place> {
    let place = sqlx::query_as::<_, Place>(
      "
      SELECT *
      FROM places
      WHERE id = $1
      ",
    )
    .bind(id)
    .fetch_one(&self.pool)
    .await?;

    Ok(place)
  }

  pub async fn get_places(&self) -> Result<Vec<Place>> {
    let places = sqlx::query_as::<_, Place>(
      "
      SELECT *
      FROM places
      ",
    )
    .fetch_all(&self.pool)
    .await?;

    Ok(places)
  }
 




  

  pub async fn create_place(&self, place: CreatePlace) -> Result<u64> {
    let tx_id = sqlx::query(
      "
      INSERT INTO places (id, name, address, city, state, country, neighborhood)
      VALUES ($1, $2, $3, $4, $5, $6, $7)
      ",
    )
    .bind(Uuid::new_v4())
    .bind(place.name)
    .bind(place.address)
    .bind(place.city)
    .bind(place.state)
    .bind(place.country)
    .bind(place.neighborhood)
    
    .execute(&self.pool)
    .await?
    .rows_affected();

    Ok(tx_id)
  }

  pub async fn update_place(&self, place: UpdatePlace, place_id: Uuid) -> Result<u64> {
    let tx_id = sqlx::query(
      "
      UPDATE places
      SET 
        name = COALESCE($1, name), 
        address = COALESCE($2, address), 
        city = COALESCE($3, city), 
        state = COALESCE($4, state), 
        country = COALESCE($5, country),
        neighborhood = COALESCE($6, neighborhood)
      WHERE id = $7
      ",
    )
    .bind(place.name)
    .bind(place.address)
    .bind(place.city)
    .bind(place.state)
    .bind(place.country)
    .bind(place.neighborhood)
    .bind(place_id)
    .execute(&self.pool)
    .await?
    .rows_affected();

    Ok(tx_id)
  }
  pub async fn delete_place(&self, id: Uuid) -> Result<u64> {
    let tx_id = sqlx::query(
      "
      DELETE FROM places
      WHERE id = $1
      ",
    )
    .bind(id)
    .execute(&self.pool)
    .await?
    .rows_affected();

    Ok(tx_id)
  }
}
