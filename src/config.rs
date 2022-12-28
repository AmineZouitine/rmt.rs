use serde::{Deserialize, Serialize};

// The configuration allows the behavior of the program concerning the trash and the added elements
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Config {
    // The file will be saved in the recycle garbage can with compression (zip) or without.
    pub compression: bool,
    pub encryption: bool,
    // Defines the set of element rules to be checked before adding an element to the trash or removing it
    pub trash: Option<Trash>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Trash {
    // Maximum size in kilobytes that the trash can contain
    pub max_size: Option<u32>,
    // maximum number of elements allowed in the trash
    pub max_element: Option<u32>,
}

impl Config {
    // The initial configuration does not compress the file and has no restrictions on the trash
    pub fn new_default_config() -> Self {
        Self {
            compression: false,
            encryption: false,
            trash: None,
        }
    }
}
