// https://developer.valvesoftware.com/wiki/KeyValues

use std::{
    collections::HashMap,
    str::FromStr,
};

use crate::error::Error;

use crate::parser::{parse, parse_file, Token};
use crate::builder::build_keyvalues;

#[derive(Debug, Clone)]
pub struct KeyValues {
    map: HashMap<String, String>,
    subkeys: HashMap<String, KeyValues>,

    //maybe todo: some sort of cache
    // All values are stored as Strings internally.
    // With get<T>(), it is parsed to T.
    // This happens every time the function is called.
    // When a value is successfully parsed, it could be stored somehow
    // And accessed from there.
    // This may require getting rid of generic get<T>
    // and replacing it with get_int, get_string, etc.
    // So maybe no, let the caller of get<T>() be the one to cache it?
    // And it's <current year>, computers can handle string parsing.
}

impl KeyValues {
    // Parse a string into KeyValues
    pub fn from_str(string: &str) -> Result<Self, Error> {
        Self::from_tokens(&parse(string)?)
    }

    // Read KeyValues from a file (like .vmt files). Error could be syntax or IO
    pub fn from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let kv = Self::from_tokens(&parse_file(path)?)?;
        Ok(kv)
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
        if let Some(t) = self.get(key) {
            t
        } else {
            T::default()
        }
    }

    pub fn get_subkey(&self, key: &String) -> Option<&KeyValues> {
        if let Some(kv) = self.subkeys.get(key) {
            Some(kv)
        } else {
            None
        }
    }

    pub fn add_value(&mut self, key: &String, value: &String) {
        self.map.insert(key.clone(), value.clone());
    }

    pub fn add_subkey(&mut self, key: &String, subkey: &Self) {
        self.subkeys.insert(key.clone(), subkey.clone());
    }


    fn from_tokens(tokens: &Vec<Token>) -> Result<Self, Error> {
        build_keyvalues(tokens)
    }
}