use ahash::RandomState;
use std::error::Error;
use std::fs;
use std::{collections::HashMap, path::PathBuf};

pub struct Data {
    map: HashMap<String, String, RandomState>,
}

impl Data {
    /// read database file
    pub fn open<P: Into<PathBuf> + std::convert::AsRef<std::path::Path>>(file: P) -> Data {
        let result = Data {
            map: HashMap::default(),
        };
        result
    }

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
