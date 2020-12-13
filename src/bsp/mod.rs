mod lump;
mod error;
mod header;

use lump::*;
use error::*;
use header::*;

use std::{
    fs::File,
    io::Read,
    string::FromUtf8Error,
};

#[derive(Debug)]
pub struct Bsp {
    pub header: Header,
    pub lumps: Vec<Vec<u8>>,
}

impl Bsp {
    pub fn from_file(path: &str) -> Result<Bsp, Box<dyn std::error::Error>> {
        let mut bytes: Vec<u8> = Vec::new();
        let mut f = File::open(path)?;
        
        if let Err(e) = f.read_to_end(&mut bytes) {
            Err(Box::new(Error::IoError(e)))
        } else {
            let res = Bsp::from_bytes(bytes)?;
            Ok(res)
        }
    }

    pub fn from_bytes(bytes: Vec<u8>) -> Result<Bsp, Error> {
        let header = Header::from_bytes(&bytes)?;
        let mut lumps: Vec<Vec<u8>> = Vec::new();

        for i in 0..LUMPS {
            lumps.push(header.lumps[i].get_data_from_bytes(&bytes)?);
        }
        
        Ok(Bsp { header, lumps })
    }
    
    // Convert the data in the Entity lump into a HashMap
    // pub fn entity_lump(&self) -> Result<Vec<HashMap<String, String>>, FromUtf8Error> {
    //     let mut map: Vec<HashMap<String, String>> = Vec::new();
    //     let string = self.entity_lump_as_string()?;

    //     Ok(map)
    // }
    
    // Convert the data in the Entity lump to a new String
    // Returns an empty string if the lump doesn't exist
    //  | VBSP guarantees that at least one entity, "worldspawn", exists
    //  | so this should really never happen (but it CAN happen!)
    // Returns a FromUtf8Error if, well, there was a problem making a UTF-8 String
    //  | 102% of the time, it's because the lump is LZMA compressed
    //  | TF2, and probably more, do this (TODO: Decompression)
    //  | Otherwise this lump should be valid ASCII, so... good luck if this happens
    pub fn entity_lump_as_string(&self) -> Result<String, FromUtf8Error> {
        String::from_utf8(self.lumps[LumpIndex::Entities as usize].clone())
    }
}

// fn parse_entity_lump_string(string: String) -> Result<Vec<HashMap<String, String>>, ()> {
//     // todo lol
// }

impl std::fmt::Display for Bsp {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f,
"Version: {}, Map Iteration: {}",
    self.header.version,
    self.header.iteration,
        )
    }
}
