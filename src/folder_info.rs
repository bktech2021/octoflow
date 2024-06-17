use anyhow::Result;

#[derive(serde::Serialize, Debug)]
pub struct Directory {
    pub path: String,
    pub childs: Vec<DirContent>,
}

#[derive(serde::Serialize, Debug)]
pub struct FolderInfo {
    name: String,
    // TODO: add later
    // content_count: u32,
}

#[derive(serde::Serialize, Debug)]
pub struct FileInfo {
    name: String,
    // TODO: add later
    // size: usize
    // part_count: usize
}

#[derive(serde::Serialize, Debug)]
pub struct SymlinkInfo {
    name: String,
}

#[derive(serde::Serialize, Debug)]
pub enum DirContent {
    Folder(FolderInfo),
    File(FileInfo),
    Symlink(SymlinkInfo),
}

impl Directory {
    pub fn from_path(path: &str) -> Result<Directory> {
        let paths = std::fs::read_dir(path)?;
        let mut contents = Vec::new();
        let mut _count: u32 = 0;
        for path in paths {
            _count += 1;
            let path = path.unwrap().path();

            if path.is_dir() {
                let info = FolderInfo {
                    name: path.to_str().unwrap().to_owned(),
                };
                contents.push(DirContent::Folder(info));
            } else if path.is_file() {
                let info = FileInfo {
                    name: path.to_str().unwrap().to_owned(),
                };
                contents.push(DirContent::File(info))
            } else {
                log::warn!("Symlinks are still out of support, ignoring.")
            }
        }

        Ok(Directory {
            path: path.to_owned(),
            childs: contents,
        })
    }
}
