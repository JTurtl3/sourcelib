// https://developer.valvesoftware.com/wiki/KeyValues
// Basically a recursive HashMap<String, String>.
// Comes with a way to parse from files and strings
// in the Source Engine KeyValues format.
// (Example: .vmt files are Key-Value files)
// All values are stored as a String in memory,
// but can be read as different types with get<T: FromStr>(key: String) -> T

pub mod keyvalues;
pub use keyvalues::*;

mod error;
pub use error::*;

mod parser;
pub use parser::*;

mod builder;