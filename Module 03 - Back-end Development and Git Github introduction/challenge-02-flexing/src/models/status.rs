use serde::Serialize;


#[derive(Debug, Serialize)]
pub struct Response<'a, T> {
  pub status: &'a str,
  #[serde(skip_serializing_if="Option::is_none")]
  pub data: Option<T>,
  #[serde(skip_serializing_if="Option::is_none")]
  pub error: Option<String>
}


impl<'a, T> Response<'a, T> {
    pub fn new_response(status: &'a str, data: Option<T> ) -> Self{
      Response { status, data, error: None }
    }

    pub fn new_error_response(status: &'a str, error: Option<String>) -> Self{
      Response { status, data: None, error }
    }
}