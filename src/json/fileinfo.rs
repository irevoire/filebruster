use super::Listing;
use anyhow::Result;
use serde::Serialize;
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Default, Serialize)]
pub struct FileInfo {
    #[serde(flatten)]
    listing: Listing,

    path: String,
    name: String,
    size: u64,
    extension: String,
    #[serde(rename = "modified")]
    mod_time: String,
    mode: u32,
    #[serde(rename = "isDir")]
    is_dir: bool,
    #[serde(rename = "type")]
    file_type: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    subtitles: Vec<String>,
    #[serde(skip_serializing_if = "String::is_empty")]
    content: String,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    checksums: HashMap<String, String>,
}

impl FileInfo {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_path(path: &Path) -> Result<Self> {
        let mut base = Self::default();
        base.build(path)?;
        Ok(base)
    }

    pub fn build(&mut self, path: &Path) -> Result<()> {
        dbg!(path);
        dbg!(path.metadata()?.is_dir());

        // let metadata = std::fs::metadata(path).unwrap();
        // self.path = path.to_str().unwrap().to_string();

        Ok(())
    }
}
