use crate::json::FileInfo;
use rocket::State;
use rocket_contrib::json::Json;
use std::path::{Path, PathBuf};

#[get("/resources/<path..>")]
pub fn get_resources(path: PathBuf, root: State<&'static Path>) -> Option<Json<FileInfo>> {
    let base = FileInfo::from_path(&root, &path).ok()?;

    Some(Json(base))
}
