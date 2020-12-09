use super::lump::LumpIndex;
use super::header::VBSP_HEADER;

#[derive(Debug)]
pub enum Error {
    UnexpectedEof,
    InvalidIdentifier(u32),
    UnsupportedVersion(u32),
    LumpMissing(LumpIndex),

    IoError(std::io::Error),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}", match self {

            Self::UnexpectedEof => "Unexpected EOF".to_string(),
            Self::InvalidIdentifier(id) => format!(
                    "Invalid Identifier '0x{:08x}' (expected 0x{:08x})",
                    id, VBSP_HEADER
                ),
            Self::UnsupportedVersion(vs) => format!(
                    "Unsupported BSP version '{}'", vs
                ),

            Self::LumpMissing(lmp) => format!(
                    "Lump is not in BSP: {:?}", lmp
                ),

            Self::IoError(e) => format!(
                    "IO Error: {}", e
                ),

        })
    }
}
