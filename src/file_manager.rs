use anyhow::Result;

use std::{
    collections::HashMap,
    fs::File,
    io::{Read, Seek, SeekFrom},
    ops::Deref,
    sync::{Arc, RwLock},
};

// TODO: lru eviction
struct FileManager {
    files: Arc<RwLock<HashMap<String, Arc<RwLock<File>>>>>,
}

impl FileManager {
    pub fn new() -> Self {
        FileManager {
            files: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    fn open_or_get(&self, filename: &str) -> Result<Arc<RwLock<File>>> {
        let files = self.files.read().unwrap();

        if let Some(file) = files.get(filename) {
            return Ok(Arc::clone(file));
        }

        drop(files); // Release read lock

        // Acquire write lock to insert new file
        let mut files = self.files.write().unwrap();

        // Double-check if another thread has inserted the file while we waited for the write lock
        if let Some(file) = files.get(filename) {
            return Ok(Arc::clone(file));
        }

        let new_file = File::open(filename)?;
        let arc_rwlock_file = Arc::new(RwLock::new(new_file));
        files.insert(filename.to_string(), Arc::clone(&arc_rwlock_file));

        Ok(arc_rwlock_file)
    }

    pub fn read_part(&self, filename: &str, offset: u64, size: usize) -> Result<Vec<u8>> {
        let file_rwlock = self.open_or_get(filename)?;
        let file = file_rwlock.read().unwrap();
        let mut file = file.deref();
        file.seek(SeekFrom::Start(offset))?;

        let mut buffer = vec![0; size];
        file.read_exact(&mut buffer)?;

        Ok(buffer)
    }
}
