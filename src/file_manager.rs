use anyhow::Result;
use std::{
    collections::HashMap,
    fs::{canonicalize, File},
    io::{Read, Seek, SeekFrom},
    ops::Deref,
    sync::{Arc, RwLock},
};

pub struct FileManager {
    files: RwLock<HashMap<String, Arc<RwLock<File>>>>,
}

impl FileManager {
    pub fn new() -> FileManager {
        FileManager {
            files: RwLock::new(HashMap::new()),
        }
    }

    fn get_or_open(&self, filename: String) -> Result<Arc<RwLock<File>>> {
        // Check if the file is already opened
        if let Some(file_arc) = self.files.read().unwrap().get(&filename) {
            return Ok(file_arc.clone());
        }

        // If the file doesn't exist, open it and add to the FileManager
        let file = File::open(filename.clone())?;
        let file_arc = Arc::new(RwLock::new(file));

        self.files
            .write()
            .unwrap()
            .insert(filename.to_string(), file_arc.clone());

        Ok(file_arc)
    }

    pub fn read_part(&self, filename: String, start: u64, mut length: usize) -> Result<Vec<u8>> {
        let filename = canonicalize(filename).unwrap().to_str().unwrap().to_owned();
        // Get or open the file
        let file_lock = self.get_or_open(filename)?;
        let file = file_lock.read().unwrap();
        let mut file = file.deref();
        let file_len = file.metadata().unwrap().len();
        if (start + length as u64) > file_len {
            length = (file_len - start) as usize;
        }
        // Seek to the specified position
        file.seek(SeekFrom::Start(start))?;

        // Read the specified length of bytes
        let mut buffer = vec![0; length];
        file.read_exact(&mut buffer)?;

        Ok(buffer)
    }
}
