use std::io::Cursor;
use std::path::PathBuf;

use axum::body::StreamBody;
use axum::headers::ContentType;
use axum::response::{self, IntoResponse};
use axum::routing::get;
use axum::{extract, Extension, Router, TypedHeader};
use rust_embed::RustEmbed;
use tera::{Context, Tera};
use tokio::io::BufReader;
use tokio_util::io::ReaderStream;

pub fn setup_router(tmpl: String) -> Router {
    Router::new()
        .route("/", get(file))
        .route("/static/:path", get(static_files))
        .layer(Extension(tmpl))
}

#[derive(RustEmbed)]
#[folder = "filebrowser/frontend/dist"]
pub struct Static;

pub async fn file(Extension(tmpl): Extension<String>) -> response::Html<String> {
    let mut tmpl_params: Context = Context::new();
    tmpl_params.insert("ReCaptcha", &false);
    tmpl_params.insert("ReCaptchaHost", "");
    tmpl_params.insert("Name", "FileBruster");
    tmpl_params.insert("StaticURL", "/static");
    tmpl_params.insert(
        "Json",
        r#"{
            "AuthMethod": "noauth",
            "BaseURL": "",
            "CSS": false,
            "DisableExternal": false,
            "EnableExec": false,
            "EnableThumbs": true,
            "LoginPage": false,
            "Name": "",
            "NoAuth": true,
            "ReCaptcha": false,
            "ResizePreview": true,
            "Signup": false,
            "StaticURL": "/static",
            "Theme": "dark",
            "Version": "2.11.0"
          }"#,
    );
    tmpl_params.insert("Theme", "dark");
    tmpl_params.insert("CSS", &false);
    response::Html(Tera::one_off(&tmpl, &tmpl_params, false).unwrap())
}

#[axum::debug_handler]
pub async fn static_files(extract::Path(path): extract::Path<PathBuf>) -> impl IntoResponse {
    dbg!("called with", &path);
    let file = Static::get(&path.to_str().unwrap()).unwrap();
    let mut file = std::str::from_utf8(&file).unwrap().to_string();

    let content_type: ContentType = match path.extension().unwrap().to_str().unwrap() {
        "js" => {
            file = file.replace("[{[ .StaticURL ]}]", "/static");
            mime::APPLICATION_JSON.into()
        }
        "html" => mime::TEXT_HTML.into(),
        "css" => mime::TEXT_CSS.into(),
        "svg" => mime::IMAGE_SVG.into(),
        _ => todo!("return a 404 not found"),
    };

    let reader = BufReader::new(Cursor::new(file));

    // convert the `AsyncRead` into a `Stream`
    let stream = ReaderStream::new(reader);
    // convert the `Stream` into an `axum::body::HttpBody`
    let body = StreamBody::new(stream);

    (TypedHeader(content_type), body)
}
