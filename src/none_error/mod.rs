use std::option::NoneError;

use crate::{JsonError, JsonErrors};

impl From<NoneError> for JsonError {
    fn from(_: NoneError) -> JsonError {
        JsonError::new(404, "None")
    }
}

impl From<NoneError> for JsonErrors {
    fn from(err: NoneError) -> JsonErrors {
        let json_error: JsonError = err.into();

        json_error.into()
    }
}
