use std::error::Error;

pub mod bond;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;
