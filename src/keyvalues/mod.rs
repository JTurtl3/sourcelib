// https://developer.valvesoftware.com/wiki/KeyValues
// Basically a recursive HashMap<String, String>.
// Comes with a way to parse from files and strings
// in the Source Engine KeyValues format.
// (Example: .vmt files are Key-Value files)
// All values are stored as a String in memory,
// but can be read as different types with get<T: FromStr>(key: String) -> T

mod error;
pub use error::*;

mod token;
pub use token::{Token, TokenKind};

mod lexer;
pub use lexer::*;

mod parser;

use std::{
    collections::HashMap,
    str::FromStr,
};

#[derive(Debug, Clone, PartialEq)]
pub struct KeyValues {
    map: HashMap<String, String>,
    subkeys: HashMap<String, KeyValues>,
}

impl KeyValues {
    // Parse a string into KeyValues
    pub fn from_str(string: &str) -> Result<Self> {
        Self::from_tokens(&tokenize(string)?)
    }

    // Read KeyValues from a file (like .vmt files). Error could be syntax or IO
    pub fn from_file(path: &str) -> std::result::Result<Self, Box<dyn std::error::Error>> {
        Ok(Self::from_tokens(&tokenize_file(path)?)?)
    }
    
    // Empty KeyValues struct
    pub fn new() -> Self {
        Self { map: HashMap::new(), subkeys: HashMap::new() }
    }

    pub fn from_pair(key: &str, value: &str) -> Self {
        let mut map: HashMap<String, String> = HashMap::new();
        map.insert(key.to_string(), value.to_string());
        
        Self { map, subkeys: HashMap::new() }
    }

    pub fn get<T: FromStr>(&self, key: &String) -> Option<T> {
        if let Ok(t) = self.map.get(key)?.parse::<T>() {
            Some(t)
        } else {
            None
        }
    }

    // Like get(), but returns the type's default value if it fails
    // (similar to GetInt, GetFloat, etc from the original KeyValues class)
    pub fn get_or_default<T: FromStr+Default>(&self, key: &String) -> T {
        self.get(key).unwrap_or(T::default())
    }

    pub fn get_subkey(&self, key: &str) -> Option<&KeyValues> {
        self.subkeys.get(key)
    }

    pub fn add_value(&mut self, key: &str, value: &str) {
        self.map.insert(key.to_string(), value.to_string());
    }

    pub fn add_subkey(&mut self, key: &str, subkey: &KeyValues) {
        self.subkeys.insert(key.to_string(), subkey.clone());
    }


    fn from_tokens(tokens: &Vec<Token>) -> Result<Self> {
        parser::parse_keyvalues(tokens)
    }
}

