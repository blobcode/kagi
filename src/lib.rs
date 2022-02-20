use std::{collections::HashMap, path::PathBuf};

mod store;

#[doc(no_inline)]
pub use crate::store::Store;

/// read database file
pub fn open<P: Into<PathBuf> + std::convert::AsRef<std::path::Path>>(file: P) -> Store {
    let path = file.into();

    let result = Store {
        map: HashMap::default(),
        file: path,
    };

    result
}
