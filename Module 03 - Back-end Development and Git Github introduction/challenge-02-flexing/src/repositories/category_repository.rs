

use sqlx::{Pool, Postgres};

use anyhow::{Ok, Result};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::category::{ Category, UpdateCategory, CreateCategory};


pub struct CategoryRepo {
  pub pool: Pool<Postgres>,
}

impl CategoryRepo {
  pub fn new(pool: PgPool) -> Self {
    Self { pool }
  }

  

  pub async fn get_category(&self, id: Uuid) -> Result<Category> {

    let category = sqlx::query_as::<_, Category>(
      "
      SELECT *
      FROM categories
      WHERE id = $1
      ",
    )
    .bind(id)
    .fetch_one(&self.pool)
    .await?;

    Ok(category)
  }

  pub async fn get_categories(&self) -> Result<Vec<Category>> {
    let category = sqlx::query_as::<_, Category>(
      "
      SELECT *
      FROM categories
      ",
    )
    .fetch_all(&self.pool)
    .await?;

    Ok(category)
  }

  pub async fn create_category(&self, category: CreateCategory) -> Result<u64> {
    let tx_id = sqlx::query(
      "
      INSERT INTO categories (id, name)
      VALUES ($1, $2)
      ",
    )
    .bind(Uuid::new_v4())
    .bind(category.name)
    .execute(&self.pool)
    .await?
    .rows_affected();

    Ok(tx_id)
  }

  pub async fn update_category(&self, category: UpdateCategory, category_id: Uuid) -> Result<u64> {
    let tx_id = sqlx::query(
      "
      UPDATE categories
      SET 
        name = COALESCE($1, name)
      WHERE id = $2
      ",
    )
    .bind(category.name)

    .bind(category_id)
    .execute(&self.pool)
    .await?
    .rows_affected();

    Ok(tx_id)
  }
  pub async fn delete_category(&self, id: Uuid) -> Result<u64> {
    let tx_id = sqlx::query(
      "
      DELETE FROM categories
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
