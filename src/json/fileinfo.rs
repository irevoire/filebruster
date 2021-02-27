use super::Listing;
use anyhow::Result;
use serde::Serialize;
use std::collections::HashMap;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

#[derive(Debug, Default, Serialize)]
pub struct FileInfo {
    #[serde(flatten)]
    listing: Option<Listing>,

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

    pub fn from_path(root: &Path, route: &Path) -> Result<Self> {
        let mut base = Self::default();
        base.build(root, route)?;
        Ok(base)
    }

    pub fn build(&mut self, root: &Path, route: &Path) -> Result<()> {
        let path = root.join(route);
        let metadata = path.metadata()?;

        self.path = route.to_str().unwrap().to_string();
        self.name = path
            .file_stem()
            .unwrap_or_else(|| path.file_name().unwrap())
            .to_str()
            .unwrap()
            .to_string();
        self.size = metadata.len();
        self.extension = path
            .extension()
            .map(|path| path.to_str().unwrap().to_string())
            .unwrap_or("".to_string());
        let time: chrono::DateTime<chrono::Utc> = metadata.modified().unwrap().into();
        self.mod_time = time.to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
        self.mode = metadata.permissions().mode();
        self.is_dir = path.is_dir();
        self.file_type = mime_guess::from_ext(&self.extension)
            .first()
            .map(|guess| guess.type_().as_str().to_string())
            .unwrap_or("".to_string());

        Ok(())
    }
}
