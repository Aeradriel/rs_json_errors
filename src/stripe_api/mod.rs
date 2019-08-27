use log::debug;
use stripe_api_crate::error::Error;

use crate::{JsonError, JsonErrors};

impl From<Error> for JsonError {
    fn from(err: Error) -> JsonError {
        match err {
            Error::Stripe(stripe_err) => {
                let mut err_string = "".to_owned();

                if let Some(message) = stripe_err.error.message {
                    err_string = format!("{}", message);
                    if let Some(param) = stripe_err.error.param {
                        err_string = format!("{} ({})", &err_string, &param);
                    }
                };

                if err_string.len() == 0 {
                    err_string = "Stripe error".to_owned();
                }

                JsonError::new(422, &err_string)
            }
            Error::Http(reqwest_err) => reqwest_err.into(),
            _ => {
                debug!("Could not convert following stripe error: {:?}", err);
                JsonError::new(500, "Unknown Stripe error")
            }
        }
    }
}

impl From<Error> for JsonErrors {
    fn from(err: Error) -> JsonErrors {
        let json_error: JsonError = err.into();

        json_error.into()
    }
}
