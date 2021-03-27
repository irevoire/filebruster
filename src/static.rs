use std::path::PathBuf;

use rocket::State;
use rocket::{
    http::ContentType,
    response::content::{Content, Html},
};
use rust_embed::RustEmbed;
use tera::Context;
use tera::Tera;

#[derive(RustEmbed)]
#[folder = "filebrowser/frontend/dist"]
pub struct Static;

#[get("/", rank = 4)]
pub fn file(tmpl: State<String>) -> Html<String> {
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
    Html(Tera::one_off(&tmpl, &tmpl_params, false).unwrap())
}

#[get("/<path..>", rank = 4)]
pub fn static_files(path: PathBuf) -> Option<Content<String>> {
    dbg!("called with", &path);
    let file = Static::get(&path.to_str()?)?;
    let file = String::from_utf8(file.to_vec()).unwrap();

    match path.extension().unwrap().to_str()? {
        "js" => Some(Content(ContentType::JavaScript, file)),
        "html" => Some(Content(ContentType::HTML, file)),
        "css" => Some(Content(ContentType::CSS, file)),
        "svg" => Some(Content(ContentType::SVG, file)),
        _ => None,
    }
}
