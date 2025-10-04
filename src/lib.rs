#![deny(unsafe_code)]

mod api;
mod app;
mod error;
mod models;

pub use app::run;
pub use error::Error;
