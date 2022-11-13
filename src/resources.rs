use std::path::PathBuf;
use std::sync::Arc;

use axum::routing::get;
use axum::{extract, response, Extension, Router};

use crate::json::FileInfo;

pub fn setup_router(path: PathBuf) -> Router {
    Router::new()
        .route("/", get(get_resources_root))
        .route("/:path", get(get_resources))
        .layer(Extension(path))
}

#[axum::debug_handler]
pub async fn get_resources_root(
    Extension(root): Extension<Arc<PathBuf>>,
) -> response::Json<Option<FileInfo>> {
    let base = FileInfo::from_path(&root, &PathBuf::new()).ok();
    response::Json(base)
}

#[axum::debug_handler]
pub async fn get_resources(
    extract::Path(path): extract::Path<PathBuf>,
    Extension(root): Extension<Arc<PathBuf>>,
) -> response::Json<Option<FileInfo>> {
    let base = FileInfo::from_path(&root, &path).ok();
    response::Json(base)
}
