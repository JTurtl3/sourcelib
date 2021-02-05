use super::error::*;
use super::lump::Lump;

use std::io::Read;

pub const VBSP_HEADER: u32 = 0x50534256;

#[derive(Debug)]
pub struct Header {
    pub version: u32,
    pub lumps: [Lump; 64],
    pub iteration: u32,
}

impl Header {
    pub fn read<T: Read>(file: &mut T) -> Result<Self> {
        read_identifier(file)?;

        let version = read_version(file)?;

        let mut lumps = [Lump::default(); 64];
        for i in 0..64 {
            lumps[i] = Lump::read(file)?;
        }

        let mut iteration = [0; 4];
        file.read(&mut iteration)?;
        let iteration = u32::from_le_bytes(iteration);

        Ok(Self { version, lumps, iteration})
    }
}

fn read_identifier<T: Read>(file: &mut T) -> Result<()> {
    // Every valid Source Engine BSP starts with "VBSP" as an unsigned 4-byte integer
    // If it's not present, the file may be corrupt or just not actually a BSP
    fn is_valid_identifier(id: u32) -> bool {
        id == VBSP_HEADER
    }

    // Read the first 4 bytes
    let mut id: [u8; 4] = [0; 4];
    file.read(&mut id)?;

    // Convert to a u32
    let id = u32::from_le_bytes(id);

    if !is_valid_identifier(id) {
        Err(Error::InvalidIdentifier(id))
    } else {
        Ok(())
    }
}

fn read_version<T: Read>(file: &mut T) -> Result<u32> {
    let mut version: [u8; 4] = [0; 4];
    file.read(&mut version)?;
    let version = u32::from_le_bytes(version);
    Ok(version)
}