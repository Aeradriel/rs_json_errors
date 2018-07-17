use rocket::http::Status;

use json_errors::JsonError;

#[derive(Deserialize)]
struct ApiError {
    error: String,
}

fn parse_body(body: &str) -> Result<ApiError, serde_json::Error> {
    serde_json::from_str(body)
}

impl<'a> From<::reqwest_crate::Response> for JsonError {
    fn from(mut resp: ::reqwest_crate::Response) -> JsonError {
        let status_int = resp.status().as_u16();
        let status = Status::new(status_int, "");
        let body = resp.text().expect("Could not read response body");

        match parse_body(&body) {
            Ok(api_err) => JsonError::new(status, &api_err.error),
            Err(_) => JsonError::new(status, &body),
        }
    }
}
