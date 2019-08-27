//! # JsonErrors
//!
//! This crate provides an easy way to handle errors from popular crates and
//! transform it into a JSON response. It is meant to be used with Rocket
//!
//! ## Diesel
//!
//! Diesel errors are supported through the `diesel` feature
//!
//! ## Reqwest
//!
//! Reqwest errors are supported through the `reqwest` feature
//!
//! ## Reqwest
//!
//! Errors from the stripe crate are supported through the `stripe_api` feature

#![feature(try_trait)]

#[cfg(feature = "diesel")]
extern crate diesel as diesel_crate;
#[cfg(any(feature = "reqwest", feature = "stripe_api"))]
extern crate reqwest as reqwest_crate;
#[cfg(feature = "stripe_api")]
extern crate stripe_api as stripe_api_crate;
#[macro_use]
extern crate serde_json;
#[cfg(feature = "reqwest")]
#[macro_use]
extern crate serde_derive;

#[cfg(feature = "diesel")]
pub mod diesel;
pub mod json_errors;
pub mod none_error;
#[cfg(any(feature = "reqwest", feature = "stripe_api"))]
pub mod reqwest;
#[cfg(feature = "stripe_api")]
pub mod stripe_api;

pub use crate::json_errors::*;
