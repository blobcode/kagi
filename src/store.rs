use ahash::RandomState;
use std::fs;
use std::{collections::HashMap, path::PathBuf};

/// main state struct
pub struct Store {
    pub map: HashMap<String, String, RandomState>,
    pub file: PathBuf,
}

impl Store {
    /// save current state to file
    //pub fn save(&self) -> Data {}

    /// insert value into database
    pub fn insert<S: Into<String>>(&mut self, key: S, value: S) {
        self.map.insert(key.into(), value.into());
    }

    /// retrive value from database
    pub fn get<S: Into<String>>(&self, key: S) -> String {
        // read from passed hashmap
        let result = self.map.get(&key.into()).unwrap().to_string();
        result
    }
}
