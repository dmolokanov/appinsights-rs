use std::error::Error;

pub mod ast;
pub mod compiler;
pub mod parser;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;
