#![allow(dead_code)] // TODO: remove this

mod common;
mod pipeline;
mod scanner;
mod vm;

pub use pipeline::{interpret, Error, Result};
