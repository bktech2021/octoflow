#[derive(serde::Deserialize, Debug)]
pub struct Command {
    pub id: usize,
    pub path: String,
    pub ask: Question,
}

#[derive(serde::Deserialize, Debug)]
pub enum Question {
    FolderInfo,
    Download(usize),
}

#[derive(serde::Serialize, Debug)]
pub struct Response<T> {
    pub to_id: usize,
    pub response: T,
}
