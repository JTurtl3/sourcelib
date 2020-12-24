// https://developer.valvesoftware.com/wiki/Valve_Texture_Format
// The proprietary image format for the Source Engine

use std::error::Error as stdError;

mod error;
pub use error::*;

mod header;
pub use header::*;

#[derive(Debug)]
pub struct Vtf {
    pub header: Header,
}

impl Vtf {
    pub fn from_file(path: &str) -> Result<Self, Box<dyn stdError>> {
        
    }

    pub fn from_bytes(bytes: &Vec<u8>) -> Result<Self, Error> {
        let header = Header::from_bytes(&bytes)?;
        Ok(Self{header})
    }
}