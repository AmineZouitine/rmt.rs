use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub compression: Option<Compression>,
    pub trash: Option<Trash>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Compression {
    pub method: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Automatique {
    pub time: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Trash {
    pub max_size: Option<u32>,
    pub max_element: Option<u32>,
}

impl Config {
    pub fn new_default_config() -> Self {
        Self {
            compression: None,
            trash: None,
        }
    }
}
