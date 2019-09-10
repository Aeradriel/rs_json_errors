use log::debug;
use stripe_crate::Error;

use crate::{JsonError, JsonErrors};

impl From<Error> for JsonError {
    fn from(err: Error) -> JsonError {
        match err {
            Error::Stripe(stripe_err) => {
                JsonError::new(422, &stripe_err.to_string())
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
