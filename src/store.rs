use bincode::{config, Decode, Encode};
use std::fs::File;
use std::io::Read;
use std::io::{BufWriter, Write};
use std::{collections::HashMap, path::PathBuf};

/// main state struct
#[derive(Encode, Decode)]
pub struct Store {
    pub map: HashMap<String, String>,
    pub file: PathBuf,
}

impl Store {
    /// sync database to file
    pub fn save(&self) {
        // encode data
        let config = config::standard();
        let encoded: Vec<u8> = bincode::encode_to_vec(&self, config).unwrap();

        // write to file
        let f = File::create(&self.file).unwrap();
        let mut f = BufWriter::new(f);
        f.write_all(&encoded).unwrap();
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

/// read database file
pub fn open<P: Into<PathBuf> + std::convert::AsRef<std::path::Path>>(file: P) -> Store {
    // load path
    let path = file.into();
    let file = path.clone();

    // read from file
    let mut encoded = Vec::new();
    let mut f = File::open(path).unwrap();
    f.read_to_end(&mut encoded).unwrap();

    // default config
    let config = config::standard();

    // initialize struct
    let data: (Store, usize) = bincode::decode_from_slice(&encoded[..], config).unwrap();

    let result = Store {
        map: data.0.map,
        file,
    };

    result
}
