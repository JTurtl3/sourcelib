use crate::lump::{Lump, LUMPS};
use crate::bsp::Error;

use std::mem::size_of;

pub const HEADER_SIZE: usize = 1036;
pub const VBSP_HEADER: u32 = 0x50534256;

#[derive(std::fmt::Debug)]
pub struct Header {
    pub identifier: u32,
    pub version: u32,
    pub lumps: [Lump; LUMPS],
    pub iteration: u32,
}

impl Header {
    pub fn from_bytes(bytes: &Vec<u8>) -> Result<Header, Error> {
        let identifier  = get_identifier(bytes)?;
        let version     = get_bsp_version(bytes)?;
        let lumps       = get_lump_info(bytes)?;
        let iteration   = get_map_version(bytes)?;
    
        Ok(Header {
            identifier, version, lumps, iteration
        })
    }
}

// The Identifier: first 4 bytes
// Should spell out 'VBSP' in ascii
// If not, the map probably isn't a valid Source Engine BSP
// Ex: Quake has IBSP identifier, GoldSrc has no identifier
fn get_identifier(bytes: &Vec<u8>) -> Result<u32, Error> {
    if let Some(identifier) = get_from_offset(bytes, 0) {
        if is_valid_identifier(identifier) {
            Ok(identifier)
        } else {
            Err(Error::InvalidIdentifier(identifier))
        }
    } else {
        Err(Error::UnexpectedEof)
    }
}

fn is_valid_identifier(id: u32) -> bool {
    return id == VBSP_HEADER;
}

// Version: 4 bytes, immediately after Identifier
// The VBSP version which the map was compiled with
// Half-Life 2, CS:Source maps vary between 19 and 20
// Most games are version 20
// Some newer ones (CS:GO, Portal 2) are 21
// Seriously, all of this is in the valve developer wiki link in lib.rs
fn get_bsp_version(bytes: &Vec<u8>) -> Result<u32, Error> {
    if let Some(version) = get_from_offset(bytes, 4) {
        Ok(version)
    } else {
        Err(Error::UnexpectedEof)
    }
}

// Lumps: 64 Lumps, 16 bytes each, 1024 bytes, immediately after Version
// Stores information on where and how big each lump is
// see lump.rs
fn get_lump_info(bytes: &Vec<u8>) -> Result<[Lump; LUMPS], Error> {
    let mut lumps = [Lump::default() ; LUMPS];
    for i in 0..LUMPS {

        if let Some(lump) = get_lump_from_offset(bytes, i * size_of::<Lump>() + 8) {
            lumps[i] = lump;
        } else {
            return Err(Error::UnexpectedEof)
        }
    }
    
    Ok(lumps)
}

fn get_lump_from_offset(bytes: &Vec<u8>, offset: usize) -> Option<Lump> {
    let mut lump = Lump::default();

    if let Some(lump_offset) = get_from_offset(bytes, offset) {
        lump.offset = lump_offset;
    } else {
        return None;
    }

    if let Some(length) = get_from_offset(bytes, offset+4) {
        lump.length = length;
    } else {
        return None;
    }

    if let Some(version) = get_from_offset(bytes, offset+8) {
        lump.version = version;
    } else {
        return None;
    }

    for i in 0..4 {
        if let Some(byte) = bytes.get(i+offset+12) {
            lump.indent_code[i] = *byte;
        } else {
            return None;
        }
    }

    Some(lump)
}

// Map iteration/version: 4 bytes, after Lumps, last 4 bytes in a header
// Every time a map is saved in Hammer, its iteration is increased
fn get_map_version(bytes: &Vec<u8>) -> Result<u32, Error> {
    if let Some(version) = get_from_offset(bytes, HEADER_SIZE-4) {
        Ok(version)
    } else {
        Err(Error::UnexpectedEof)
    }
}

fn get_from_offset(bytes: &Vec<u8>, offset: usize) -> Option<u32> {
    
    if bytes.len() <= offset+4 {
        return None;
    }
    
    let mut arr = [0; 4];
    for i in 0..4 {
        arr[i] = bytes[i+offset];
    }

    Some(u32::from_le_bytes(arr))
}