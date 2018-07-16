#![feature(extern_prelude)]

extern crate diesel;
extern crate regex;
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
extern crate reqwest;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

pub mod json_errors;
