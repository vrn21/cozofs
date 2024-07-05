pub struct DirectoryFS {
    pub uuid: String,
    pub name: String,
    pub path: String,
    pub parent: String,
    pub entry: String,
}

pub struct FileFS {
    pub uuid: String,
    pub name: String,
    pub path: String,
    pub parent: String,
    pub entry: String,
}

pub enum entry {
    File,
    Directory,
}
