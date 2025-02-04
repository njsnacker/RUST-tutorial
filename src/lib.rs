#![warn(clippy::all, rust_2018_idioms)]
#![allow(non_snake_case)]

mod app;
pub use app::SerialApp;
// 프로토콜 모듈 사용
mod protocol;
