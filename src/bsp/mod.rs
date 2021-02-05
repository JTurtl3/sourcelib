mod lump;
mod error;
mod header;

pub use lump::*;
pub use error::*;
pub use header::*;

use std::fs::File;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;

use std::string::FromUtf8Error;

#[derive(Debug)]
pub struct Bsp {
    pub version: u32,
    pub lumps: [Lump; 64],
    pub iteration: u32,
    file: File,
}

impl Bsp {
    pub fn from_file(path: &str) -> Result<Self> {
        let mut file = File::open(path)?;
        let header = Header::read(&mut file)?;
        
        Ok(Self {
            version: header.version,
            lumps: header.lumps,
            iteration: header.iteration,
            file
        })
    }

    pub fn get_lump_data(&mut self, index: LumpIndex) -> Option<Vec<u8>> {
        let lump = self.lumps[index as usize];
        if lump.exists() {
            let mut v = Vec::with_capacity(lump.length as usize);
            for _ in 0..v.capacity() {
                v.push(0);
            }
            if let Ok(_new_pos) = self.file.seek(SeekFrom::Start(lump.offset as u64)) {
                if let Ok(_bytes_read) = self.file.read(&mut v) {
                    Some(v)
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    // Convert the data in the Entity lump to a new String
    // Returns an empty string if the lump doesn't exist
    //  | VBSP guarantees that at least one entity, "worldspawn", exists
    //  | so this should really never happen (but it CAN happen!)
    // Returns a FromUtf8Error if there was a problem making a UTF-8 String
    //  | 102% of the time, it's because the lump is LZMA compressed
    //  | TF2, and probably more, do this (TODO: Decompression)
    //  | Otherwise this lump should be valid ASCII, so... good luck if this happens
    pub fn entity_lump_as_string(&mut self) -> std::result::Result<String, FromUtf8Error> {
        if let Some(data) = self.get_lump_data(LumpIndex::Entities) {
            let mut s = String::from_utf8(data)?;
            // Remove the trailing null
            s.pop();
            Ok(s)
        } else {
            Ok("".to_string())
        }

    }
}

impl std::fmt::Display for Bsp {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        write!(f,
"BSP Version: {}, Map Iteration: {}",
    self.version,
    self.iteration,
        )
    }
}
