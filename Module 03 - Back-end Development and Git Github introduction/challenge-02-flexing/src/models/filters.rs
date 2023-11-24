use anyhow::{Ok, Result};
use axum::extract::Query;
use chrono::NaiveDate;
use serde::Deserialize;
use sqlx::types::Uuid;

#[derive(Debug, Deserialize)]
pub struct QueryParams {
  pub places: Option<String>,
  pub categories: Option<String>,
  pub dates: Option<String>,
}

impl Default for QueryParams {
  fn default() -> Self {
    Self {
      places: None,
      categories: None,
      dates: None,
    }
  }
}
#[derive(Debug)]
pub struct Filters {
  pub categories: Option<Vec<Uuid>>,
  pub places: Option<Vec<Uuid>>,
  pub dates: Option<Vec<NaiveDate>>,
}

//TODO check will panic
impl Filters {
  pub fn parse_query_params(query_params: Query<QueryParams>) -> Result<Filters> {
    let categories = Self::parse_uuid_list(&query_params.categories)?;
    let places = Self::parse_uuid_list(&query_params.places)?;
    let dates = Self::parser_dates(&query_params.dates)?;
    Ok(Filters {
      categories,
      places,
      dates,
    })
  }

  fn parse_uuid_list(uuid_list: &Option<String>) -> Result<Option<Vec<Uuid>>> {
    let list: Option<Vec<Uuid>> = match uuid_list {
      Some(dts) => Some(
        dts
          .split(",")
          .filter_map(|i| Uuid::try_parse(i).ok())
          .collect(),
      ),
      None => None,
    };
    println!("{:?}", list);
    return Ok(list);
  }

  fn parser_dates(dates: &Option<String>) -> Result<Option<Vec<NaiveDate>>> {
    let dates: Option<Vec<NaiveDate>> = match dates {
      Some(dts) => Some(
        dts
          .split(",")
          .filter_map(|date| Self::parse_date(date).ok())
          .collect(),
      ),
      None => None,
    };
    // println!("{:?}", dates);
    Ok(dates)
  }

  fn parse_date(date: &str) -> Result<NaiveDate> {
    let parsed_date = NaiveDate::parse_from_str(date, "%Y-%m-%d")?;
    // println!("{}", parsed_date);
    Ok(parsed_date)
  }

  pub fn check_valid_vec<T>(v: &Option<Vec<T>>) -> Result<&Vec<T>> {
    if let Some(vv) = v {
      if vv.len() > 0 {
        return Ok(vv);
      }
    }
    Err(anyhow::Error::msg("Invalid"))
  }

  pub fn check_all_valid(&self) -> Vec<bool> {
    // n-1 not need to check all

    let mut r: Vec<bool> = Vec::new();

    r.push(Self::check_valid_vec(&self.places).is_err());
    r.push(Self::check_valid_vec(&self.categories).is_err());
    // r.push(Self::check_valid_vec(&self.dates).is_err());

    return r;
  }
}
