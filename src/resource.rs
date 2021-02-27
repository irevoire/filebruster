use crate::json::FileInfo;
use rocket::http::Status;
use rocket::State;
use std::path::{Path, PathBuf};

#[get("/resources/<path..>")]
pub fn get_resources(path: PathBuf, root: State<&'static Path>) -> Result<String, Status> {
    let path = root.join(path);

    let base = FileInfo::from_path(&path).map_err(|_| Status::NotFound)?;

    Ok(path.to_str().unwrap().to_string())
}
