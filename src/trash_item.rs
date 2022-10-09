use std::fmt;

#[derive(Debug, PartialEq)]
pub struct TrashItem {
    pub id: i8,
    pub name: String,
    pub hash: String,
    pub path: String,
    pub date: String,
    pub real_size: u64,
    pub compression_size: Option<u64>,
    pub is_folder: bool,
}

impl TrashItem {
    pub fn new(
        name: String,
        hash: String,
        path: String,
        date: String,
        real_size: u64,
        compression_size: Option<u64>,
        is_folder: bool,
    ) -> Self {
        Self {
            id: -1,
            name,
            hash,
            path,
            date,
            real_size,
            compression_size,
            is_folder,
        }
    }
}

impl fmt::Display for TrashItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let emoji = if self.is_folder { "📁" } else {"📄"};

        write!(
            f,
            " {} date: {}  name: {}  inital_path: {} size: {}",
            emoji, self.date, self.name, self.path, self.real_size,
        )
    }
}
