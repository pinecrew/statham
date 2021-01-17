use std::error::Error;
use std::io;

#[derive(Debug)]
pub enum JsonError {
    Io(io::Error),
    Line(u32, u32),
}

impl Error for JsonError {}
