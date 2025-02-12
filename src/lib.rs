#![warn(clippy::all, rust_2018_idioms)]
#![allow(non_snake_case)]

mod app;
mod protocol;
mod serial;
pub use app::SerialApp;
