#[cfg(feature = "diesel")]
extern crate diesel as diesel_crate;
#[cfg(feature = "reqwest")]
extern crate reqwest as reqwest_crate;
#[macro_use]
extern crate serde_json;
#[cfg(feature = "reqwest")]
#[macro_use]
extern crate serde_derive;

#[cfg(feature = "diesel")]
pub mod diesel;
pub mod json_errors;
#[cfg(feature = "reqwest")]
pub mod reqwest;

pub use crate::json_errors::*;
