use std::path::Path;

use anyhow::Result;
use serde::Serialize;

use super::FileInfo;

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

    pub fn build(&mut self, root: &Path, route: &Path) -> Result<()> {
        let path = root.join(route);
        let items: Vec<FileInfo> = path
            .read_dir()?
            .map(|filename| {
                FileInfo::from_path_without_listing(
                    root,
                    &route.join(filename.unwrap().file_name()),
                )
                .unwrap()
            })
            .collect();

        items.iter().for_each(|fileinfo| match fileinfo.is_dir {
            true => self.num_dirs += 1,
            false => self.num_files += 1,
        });

        self.items = items;

        Ok(())
    }

    pub fn from_path(root: &Path, route: &Path) -> Result<Self> {
        let mut listing = Self::default();
        listing.build(root, route)?;

        Ok(listing)
    }
}
