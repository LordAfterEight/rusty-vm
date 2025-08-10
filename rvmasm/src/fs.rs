pub struct FileSystem {
    pub startpoint: u16,
    pub content: Vec<File>,
    pub size: usize,
    pub size_initialized: bool,
    pub offset_ptr: usize,
}

pub struct File {
    pub name: String,
    pub words: Vec<u16>,
    pub fs_index: usize,
    pub size: usize
}

impl File {
    pub fn new(name: String, fs_index: usize, size: usize) -> Self {
        Self {
            name,
            words: Vec::new(),
            fs_index,
            size,
        }
    }
}

impl FileSystem {
    pub fn new(startpoint: usize) -> Self {
        Self {
            startpoint: startpoint as u16,
            content: Vec::new(),
            size: 0,
            size_initialized: false,
            offset_ptr: 0,
        }
    }

    pub fn get_file_by_index(&self, index: usize) -> Option<&File> {
        self.content.get(index)
    }

    pub fn total_size(&self) -> usize {
        self.size
    }
}
