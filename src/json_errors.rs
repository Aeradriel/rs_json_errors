use rocket::http::ContentType;
use rocket::http::Status;
use rocket::request::Request;
use rocket::response::{self, Responder};
use rocket_contrib::json::Json;
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
    pub status: Status,
    pub description: String,
    pub body: Value,
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

impl<'r> Responder<'r> for JsonError {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        let mut res = Json(self.body).respond_to(req).unwrap();
        res.set_status(self.status);
        res.set_header(ContentType::JSON);
        Ok(res)
    }
}
