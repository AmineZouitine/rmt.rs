#[derive(Debug, PartialEq)]
pub struct TrashItem {
    pub id: i8,
    pub name: String,
    pub hash: String,
    pub path: String,
    pub date: String,
    pub real_size: u8,
    pub compression_method: Option<String>,
    pub compression_size: Option<u8>,
}

impl TrashItem {
    pub fn new(
        id: i8,
        name: String,
        hash: String,
        path: String,
        date: String,
        real_size: u8,
        compression_method: Option<String>,
        compression_size: Option<u8>,
    ) -> Self {
        Self {
            id,
            name,
            hash,
            path,
            date,
            real_size,
            compression_method,
            compression_size,
        }
    }
}
