use super::Listing;
use anyhow::Result;
use serde::Serialize;
use std::collections::HashMap;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

#[derive(Debug, Default, Serialize)]
pub struct FileInfo {
    #[serde(flatten)]
    pub(super) listing: Option<Listing>,

    pub(super) path: String,
    pub(super) name: String,
    pub(super) size: u64,
    pub(super) extension: String,
    #[serde(rename = "modified")]
    pub(super) mod_time: String,
    pub(super) mode: u32,
    #[serde(rename = "isDir")]
    pub(super) is_dir: bool,
    #[serde(rename = "type")]
    pub(super) file_type: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(super) subtitles: Vec<String>,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub(super) content: String,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub(super) checksums: HashMap<String, String>,
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

    pub(super) fn from_path_without_listing(root: &Path, route: &Path) -> Result<Self> {
        let mut base = Self::default();
        base.build_without_listing(root, route)?;
        Ok(base)
    }

    pub(super) fn build_without_listing(&mut self, root: &Path, route: &Path) -> Result<()> {
        let path = root.join(route);
        let metadata = path.metadata()?;

        self.is_dir = path.is_dir();

        if self.is_dir {
            self.name = path.file_name().unwrap().to_str().unwrap().to_string();
        } else {
            self.name = path.file_stem().unwrap().to_str().unwrap().to_string();
            self.extension = path
                .extension()
                .map(|path| path.to_str().unwrap().to_string())
                .unwrap_or("".to_string());
            self.file_type = mime_guess::from_ext(&self.extension)
                .first()
                .map(|guess| guess.type_().as_str().to_string())
                .unwrap_or("fuck you".to_string());
        }

        self.path = format!("/{}", route.display());
        self.size = metadata.len();
        let time: chrono::DateTime<chrono::Utc> = metadata.modified().unwrap().into();
        self.mod_time = time.to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
        self.mode = metadata.permissions().mode();

        Ok(())
    }

    pub fn build(&mut self, root: &Path, route: &Path) -> Result<()> {
        self.build_without_listing(root, route)?;
        if self.is_dir {
            self.listing = Some(Listing::from_path(root, route)?);
        }
        Ok(())
    }
}
