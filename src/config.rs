use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub compression: bool,
    pub trash: Option<Trash>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Trash {
    pub max_size: Option<u32>,
    pub max_element: Option<u32>,
}

impl Config {
    pub fn new_default_config() -> Self {
        Self {
            compression: false,
            trash: None,
        }
    }
}
