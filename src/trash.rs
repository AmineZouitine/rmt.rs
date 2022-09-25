struct Trash {
    id: i8,
    name: String,
    hash: String,
    path: String,
    date: String,
    real_size: u8,
    compression: Option<Compression>,
}

struct Compression {
    compression_method: String,
    compression_size: u8,
}

impl Trash {
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