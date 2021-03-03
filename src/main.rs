#![feature(proc_macro_hygiene, decl_macro)]

mod json;
mod resources;

#[macro_use]
extern crate rocket;

use rocket_contrib::templates::Template;
use std::path::Path;
use rocket_contrib::serve::StaticFiles;
use rocket::State;
use std::collections::HashMap;

#[get("/login")]
fn login() -> &'static str {
    "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VyIjp7ImlkIjoxLCJsb2NhbGUiOiJlbiIsInZpZXdNb2RlIjoibW9zYWljIiwic2luZ2xlQ2xpY2siOmZhbHNlLCJwZXJtIjp7ImFkbWluIjpmYWxzZSwiZXhlY3V0ZSI6dHJ1ZSwiY3JlYXRlIjpmYWxzZSwicmVuYW1lIjpmYWxzZSwibW9kaWZ5IjpmYWxzZSwiZGVsZXRlIjpmYWxzZSwic2hhcmUiOnRydWUsImRvd25sb2FkIjp0cnVlfSwiY29tbWFuZHMiOltdLCJsb2NrUGFzc3dvcmQiOmZhbHNlLCJoaWRlRG90ZmlsZXMiOmZhbHNlfSwiZXhwIjoxNjE0NDMxMDAzLCJpYXQiOjE2MTQ0MjM4MDMsImlzcyI6IkZpbGUgQnJvd3NlciJ9.BNpqWQIPIK6_nEFrIpAHYTTbfOhwex_QyWzfZkcf4a8"
}

#[get("/renew")]
fn renew() -> &'static str {
    login()
}

#[get("/**", rank = 4)]
fn file(tmplParams: State<HashMap<String, String>>) -> Template {
    let mut tmplParams: HashMap<String, String> = HashMap::new();
    tmplParams.insert("ReCaptcha".to_string(), "false".to_string());
    tmplParams.insert("ReCaptchaHost".to_string(), "".to_string());
    tmplParams.insert("Name".to_string(), "FileBruster".to_string());
    tmplParams.insert("StaticURL".to_string(), "/static".to_string());
    tmplParams.insert("Json".to_string(), r#"{
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
          }"#.to_string());
    tmplParams.insert("Theme".to_string(), "dark".to_string());
    tmplParams.insert("CSS".to_string(), "false".to_string());
    Template::render("index", &tmplParams)
}

fn main() {
    let root = Box::new(std::env::current_dir().unwrap());
    let root: &'static Path = Box::leak(root);

    rocket::ignite()
        .manage(root)
        .mount("/api", routes![login, renew, resources::get_resources])
        .mount("/static", StaticFiles::from("filebrowser/frontend/dist"))
        .mount("/", routes![file])
        .attach(Template::custom(|engines| {
            let mut file = String::from_utf8(std::fs::read("filebrowser/frontend/dist/index.html").unwrap()).unwrap();
            let tmpl = file.replace("[{[ .", "{{ ")
                           .replace("]}]", "}}");
            engines.tera.add_raw_template("index", &tmpl);
        }))
        .launch();
}
