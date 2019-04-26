use crate::json_errors::{JsonError, JsonErrors};

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
        let body = resp.text().expect("Could not read response body");

        match parse_body(&body) {
            Ok(api_err) => JsonError::new(status_int, &api_err.error),
            Err(_) => JsonError::new(status_int, &body),
        }
    }
}

impl From<::reqwest_crate::Response> for JsonErrors {
    fn from(err: ::reqwest_crate::Response) -> JsonErrors {
        let json_error: JsonError = err.into();

        json_error.into()
    }
}

impl<'a> From<::reqwest_crate::Error> for JsonError {
    fn from(err: ::reqwest_crate::Error) -> JsonError {
        let status = if let Some(status) = err.status() {
            status.as_u16()
        } else {
            500
        };

        if err.is_serialization() {
            JsonError::new(status, "Serialization error")
        } else {
            JsonError::new(status, format!("Unknown error: {}", err))
        }
    }
}

impl From<::reqwest_crate::Error> for JsonErrors {
    fn from(err: ::reqwest_crate::Error) -> JsonErrors {
        let json_error: JsonError = err.into();

        json_error.into()
    }
}
