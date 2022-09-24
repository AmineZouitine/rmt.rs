use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    compression: Option<Compression>,
    flush: Flush,
    trash: Option<Trash>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Compression {
    method: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Flush {
    warning: bool,
    automatique: Option<Automatique>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Automatique {
    time: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Trash {
    max_size: Option<u32>,
    max_element: Option<u32>,
}

impl Config {
    pub fn new_default_config() -> Self {
        Self {
            compression: None,
            flush: Flush::new(true, None),
            trash: None,
        }
    }
}

impl Flush {
    fn new(warning: bool, automatique: Option<Automatique>) -> Self {
        Self {
            warning,
            automatique,
        }
    }
}
