use super::FileInfo;
use serde::Serialize;

#[derive(Debug, Default, Serialize)]
pub struct Listing {
    items: Vec<FileInfo>,
    #[serde(rename = "numDirs")]
    num_dirs: i32,
    #[serde(rename = "numFiles")]
    num_files: i32,
    sorting: Sorting,
}

#[derive(Debug, Default, Serialize)]
pub struct Sorting {
    by: String,
    asc: bool,
}

impl Listing {
    pub fn new() -> Self {
        Self::default()
    }
}
