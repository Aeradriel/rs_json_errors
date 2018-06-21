use diesel::result::{DatabaseErrorInformation, DatabaseErrorKind, Error::DatabaseError};
use regex::Regex;
use rocket::http::ContentType;
use rocket::http::Status;
use rocket::request::Request;
use rocket::response::{self, Responder};
use rocket_contrib::Json;

#[derive(Debug)]
pub struct JsonErrors(pub Vec<JsonError>, pub Status);

impl<'r> Responder<'r> for JsonErrors {
  fn respond_to(self, req: &Request) -> response::Result<'r> {
    let errors = self.0;
    let vec_errors = errors
      .into_iter()
      .map(|error| error.description)
      .collect::<Vec<String>>();

    let errors_description = vec_errors.join("\n");

    let body = Json(json!({
      "error": errors_description,
      "errors": vec_errors 
    }));

    let mut res = body.respond_to(req).unwrap();
    res.set_status(self.1);
    res.set_header(ContentType::JSON);
    Ok(res)
  }
}

#[derive(Debug)]
pub struct JsonError {
  status: Status,
  description: String,
  body: Json,
}

impl JsonError {
  pub fn new(status: Status, description: &str) -> Self {
    JsonError {
      status,
      description: String::from(description),
      body: Json(json!({ "error": description.to_string() })),
    }
  }
}

impl<'b> From<&'b diesel::result::Error> for JsonError {
  fn from(err: &'b diesel::result::Error) -> JsonError {
    let mut origin = String::from("An error occured");
    let mut cause = "";
    let mut status = Status::InternalServerError;

    if let DatabaseError(kind, infos) = err {
      match kind {
        DatabaseErrorKind::UniqueViolation => {
          cause = "already exists";
          status = Status::UnprocessableEntity;
          if let Some(name) = column_from_database_error_infos(infos) {
            origin = name;
          }
        }
        DatabaseErrorKind::ForeignKeyViolation => {
          cause = "violates foreign key";
          status = Status::UnprocessableEntity;
          if let Some(name) = column_from_database_error_infos(infos) {
            origin = name;
          }
        }
        _ => cause = "during a database transaction",
      }
    }
    if cause.is_empty() {
      JsonError::new(status, &origin)
    } else {
      JsonError::new(status, &format!("{} {}", origin, cause))
    }
  }
}

impl<'r> Responder<'r> for JsonError {
  fn respond_to(self, req: &Request) -> response::Result<'r> {
    let mut res = self.body.respond_to(req).unwrap();
    res.set_status(self.status);
    res.set_header(ContentType::JSON);
    Ok(res)
  }
}

fn column_from_database_error_infos(
  infos: &Box<DatabaseErrorInformation + Send + Sync>,
) -> Option<String> {
  if let Some(column) = infos.column_name() {
    Some(String::from(column))
  } else if let Some(constraint) = infos.constraint_name() {
    let mut origin = String::from(constraint);

    if let Some(table_name) = infos.table_name() {
      if let Ok(regex) = Regex::new(&format!(r"{}_(.+)_key", table_name)) {
        origin = String::from(regex.replace(constraint, "$1"));
      }
    }
    Some(origin)
  } else {
    None
  }
}
