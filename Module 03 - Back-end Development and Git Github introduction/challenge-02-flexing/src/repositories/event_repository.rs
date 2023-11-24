use sqlx::{Pool, Postgres};

use anyhow::{Ok, Result};
use sqlx::{PgPool, QueryBuilder, Row};
use uuid::Uuid;

use crate::models::{
  event::{CreateEvent, Event, EventUpdate},
  filters::Filters,
};

pub struct EventRepository {
  pub pool: Pool<Postgres>,
}

impl EventRepository {
  pub fn new(pool: PgPool) -> Self {
    Self { pool }
  }

  // TODO 1st dateTime wrong is naive, 2nd cannot bind list, 3rd where is not good, 4th
  pub async fn get_filtered_events(&self, filters: Filters) -> Result<Vec<Event>> {
    let mut query: QueryBuilder<Postgres> = QueryBuilder::new("SELECT * FROM events");

    let valid = filters.check_all_valid();

    if let core::result::Result::Ok(places) = Filters::check_valid_vec(&filters.places) {
      query.push("WHERE place_id IN (");
      let mut separated = query.separated(",");

      for p in places.iter() {
        separated.push_bind(p);
      }
      separated.push_unseparated(")");
    }

    if let core::result::Result::Ok(categories) = Filters::check_valid_vec(&filters.categories) {
      if valid[0] == true {
        query.push(" WHERE");
      } else {
        query.push(" AND");
      }

      query.push(" category_id IN (");

      let mut separated = query.separated(", ");

      for c in categories.iter() {
        separated.push_bind(c);
      }
      separated.push_unseparated(")");
    }

    if let core::result::Result::Ok(dates) = Filters::check_valid_vec(&filters.dates) {
      if valid[0] == true && valid[1] == true {
        query.push(" WHERE");
      } else {
        query.push(" AND");
      }

      query.push(" date >= ");
      query.push_bind(dates[0]);

      if dates.len() > 1 {
        query.push(" AND date <= ");
        query.push_bind(dates[1]);
      }
    }

    // println!("{}", query.sql());
    let res: Vec<Event> = query
      .build()
      .fetch_all(&self.pool)
      .await?
      .into_iter()
      .map(|row| Event {
        id: row.get("id"),
        name: row.get("name"),
        description: row.get("description"),
        date: row.get("date"),
        category_id: row.get("category_id"),
        place_id: row.get("place_id"),
      })
      .collect();

    // println!("{}", res.len());
    Ok(res)
  }

  pub async fn get_event(&self, id: Uuid) -> Result<Event> {
    println!("{:?}", id);
    let event = sqlx::query_as::<_, Event>(
      "
      SELECT *
      FROM events
      WHERE id = $1
      ",
    )
    .bind(id)
    .fetch_one(&self.pool)
    .await?;

    Ok(event)
  }

  pub async fn get_events(&self) -> Result<Vec<Event>> {
    let events = sqlx::query_as::<_, Event>(
      "
      SELECT *
      FROM events
      ",
    )
    .fetch_all(&self.pool)
    .await?;

    Ok(events)
  }

  pub async fn create_event(&self, event: CreateEvent) -> Result<u64> {
    let tx_id = sqlx::query(
      "
      INSERT INTO events (id, name, description, date, category_id, place_id)
      VALUES ($1, $2, $3, $4, $5, $6)
      ",
    )
    .bind(Uuid::new_v4())
    .bind(event.name)
    .bind(event.description)
    .bind(event.date)
    .bind(event.category_id)
    .bind(event.place_id)
    .execute(&self.pool)
    .await?
    .rows_affected();

    Ok(tx_id)
  }

  pub async fn update_event(&self, event: EventUpdate, event_id: Uuid) -> Result<u64> {
    let tx_id = sqlx::query(
      "
      UPDATE events
      SET 
        name = COALESCE($1, name), 
        description = COALESCE($2, description), 
        date = COALESCE($3, date), 
        category_id = COALESCE($4, category_id), 
        place_id = COALESCE($5, place_id)
      WHERE id = $6
      ",
    )
    .bind(event.name)
    .bind(event.description)
    .bind(event.date)
    .bind(event.category_id)
    .bind(event.place_id)
    .bind(event_id)
    .execute(&self.pool)
    .await?
    .rows_affected();

    Ok(tx_id)
  }
  pub async fn delete_event(&self, id: Uuid) -> Result<u64> {
    let tx_id = sqlx::query(
      "
      DELETE FROM events
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
