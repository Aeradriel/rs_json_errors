use diesel::result::{
    DatabaseErrorInformation, DatabaseErrorKind, Error::DatabaseError, Error::NotFound,
};
use regex::Regex;
use rocket::http::ContentType;
use rocket::http::Status;
use rocket::request::Request;
use rocket::response::{self, Responder};
use rocket_contrib::Json;
use serde_json::Value;

#[derive(Debug, PartialEq)]
pub struct JsonErrors(pub Vec<JsonError>, pub Status);

impl<'r> Responder<'r> for JsonErrors {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        let errors = self.0;
        let vec_errors = errors
            .into_iter()
            .map(|error| error.description)
            .collect::<Vec<String>>();

        let errors_description = vec_errors.join("\n");

        let body = json!({
            "error": errors_description,
            "errors": vec_errors
        });

        let mut res = Json(body).respond_to(req).unwrap();
        res.set_status(self.1);
        res.set_header(ContentType::JSON);
        Ok(res)
    }
}

#[derive(Debug, PartialEq)]
pub struct JsonError {
    status: Status,
    description: String,
    body: Value,
}

impl JsonError {
    pub fn new(status: Status, description: &str) -> Self {
        JsonError {
            status,
            description: String::from(description),
            body: json!({ "error": description.to_string() }),
        }
    }
}

impl<'b> From<JsonError> for JsonErrors {
    fn from(err: JsonError) -> JsonErrors {
        let status = err.status.clone();

        JsonErrors(vec![err], status)
    }
}

impl<'b> From<&'b diesel::result::Error> for JsonError {
    fn from(err: &'b diesel::result::Error) -> JsonError {
        let ocs = match err {
            DatabaseError(kind, infos) => match kind {
                DatabaseErrorKind::UniqueViolation => (
                    column_from_database_error_infos(infos),
                    String::from("already exists"),
                    Status::UnprocessableEntity,
                ),
                DatabaseErrorKind::ForeignKeyViolation => (
                    column_from_database_error_infos(infos),
                    String::from("violates foreign key"),
                    Status::UnprocessableEntity,
                ),
                _ => (
                    Some(String::from("Database error: ")),
                    format!("{:?}", err),
                    Status::InternalServerError,
                ),
            },
            NotFound => (None, String::from("Not found"), Status::NotFound),
            err => (
                Some(String::from("Database error: ")),
                format!("{:?}", err),
                Status::InternalServerError,
            ),
        };

        let mut error = vec![];

        if let Some(origin) = ocs.0 {
            error.push(origin);
        }
        error.push(ocs.1);

        JsonError::new(ocs.2, &error.join(" "))
    }
}

impl<'r> Responder<'r> for JsonError {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        let mut res = Json(self.body).respond_to(req).unwrap();
        res.set_status(self.status);
        res.set_header(ContentType::JSON);
        Ok(res)
    }
}

fn column_from_database_error_infos(
    infos: &Box<DatabaseErrorInformation + Send + Sync>,
) -> Option<String> {
    if let Some(column) = infos.column_name() {
        return Some(String::from(column));
    } else if let Some(constraint) = infos.constraint_name() {
        let mut origin = String::from(constraint);

        if let Some(table_name) = infos.table_name() {
            if let Ok(regex) = Regex::new(&format!(r"{}_(.+)_key", table_name)) {
                origin = String::from(regex.replace(constraint, "$1"));
            }
        }
        return Some(origin);
    }
    None
}

use hyper::rt::Future;
use hyper::rt::Stream;

impl<'a> From<hyper::Response<hyper::Body>> for JsonError {
    fn from(mut resp: hyper::Response<hyper::Body>) -> JsonError {
        let status_int = resp.status().as_str().parse().unwrap();
        let status = Status::new(status_int, "");
        let body_str = resp
            .body_mut()
            .concat2()
            .map(|chunk| {
                let v = chunk.to_vec();

                String::from_utf8_lossy(&v).to_string()
            })
            .wait()
            .expect("Could not read response body");

        JsonError::new(status, &body_str)
    }
}
