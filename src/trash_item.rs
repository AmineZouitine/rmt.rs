#[derive(Debug)]
pub struct TrashItem {
    pub id: i8,
    pub name: String,
    pub hash: String,
    pub path: String,
    pub date: String,
    pub real_size: u8,
    pub compression: Option<Compression>,
}

#[derive(Debug)]
pub struct Compression {
    compression_method: String,
    compression_size: u8,
}

impl TrashItem {
    pub fn new(
        id: i8,
        name: String,
        hash: String,
        path: String,
        date: String,
        real_size: u8,
        compression: Option<Compression>,
    ) -> Self {
        Self {
            id,
            name,
            hash,
            path,
            date,
            real_size,
            compression,
        }
    }
}

impl Compression {
    pub fn new(compression_method: String, compression_size: u8) -> Self {
        Self {
            compression_method,
            compression_size,
        }
    }
}
