use diesel_crate::result::{
    DatabaseErrorInformation, DatabaseErrorKind, Error::DatabaseError, Error::NotFound,
};
use regex::Regex;
use rocket::http::Status;

use crate::json_errors::{JsonError, JsonErrors};

impl From<::diesel_crate::result::Error> for JsonError {
    fn from(err: ::diesel_crate::result::Error) -> JsonError {
        let ocs = match err {
            DatabaseError(kind, ref infos) => match kind {
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

        JsonError::from_status(ocs.2, &error.join(" "))
    }
}

impl From<::diesel_crate::result::Error> for JsonErrors {
    fn from(err: ::diesel_crate::result::Error) -> JsonErrors {
        let json_error: JsonError = err.into();

        json_error.into()
    }
}

fn column_from_database_error_infos(
    infos: &Box<dyn DatabaseErrorInformation + Send + Sync>,
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
