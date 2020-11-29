// https://developer.valvesoftware.com/wiki/KeyValues

use std::{
    collections::HashMap,
    str::FromStr,
};

pub struct KeyValues {
    map: HashMap<String, String>,
    subkeys: HashMap<String, KeyValues>,
}

impl KeyValues {
    // pub fn from_str(string: &str) -> Self {
        
    // }

    // pub fn from_vec(vector: &Vec<String>) -> Self {

    // }

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
}

