use tera::Context;
use rocket::response::content::{Html};
use rocket::State;
use tera::Tera;
use rust_embed::RustEmbed;

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
