use ahash::RandomState;
use std::fs;
use std::{collections::HashMap, path::PathBuf};

/// main state struct
pub struct Store {
    map: HashMap<String, String, RandomState>,
    file: PathBuf,
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

/// read database file
pub fn open<P: Into<PathBuf> + std::convert::AsRef<std::path::Path>>(file: P) -> Store {
    let path = file.into();

    let result = Store {
        map: HashMap::default(),
        file: path,
    };

    result
}
