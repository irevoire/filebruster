#![feature(proc_macro_hygiene, decl_macro)]

mod json;
mod resources;

#[macro_use]
extern crate rocket;

use rocket::State;
use rocket::response::content::Html;
use std::path::Path;
use rocket_contrib::serve::StaticFiles;
use tera::Context;
use tera::Tera;

#[post("/login")]
fn login() -> &'static str {
    "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VyIjp7ImlkIjoxLCJsb2NhbGUiOiJlbiIsInZpZXdNb2RlIjoibW9zYWljIiwic2luZ2xlQ2xpY2siOmZhbHNlLCJwZXJtIjp7ImFkbWluIjpmYWxzZSwiZXhlY3V0ZSI6dHJ1ZSwiY3JlYXRlIjpmYWxzZSwicmVuYW1lIjpmYWxzZSwibW9kaWZ5IjpmYWxzZSwiZGVsZXRlIjpmYWxzZSwic2hhcmUiOnRydWUsImRvd25sb2FkIjp0cnVlfSwiY29tbWFuZHMiOltdLCJsb2NrUGFzc3dvcmQiOmZhbHNlLCJoaWRlRG90ZmlsZXMiOmZhbHNlfSwiZXhwIjoxNjE0NDMxMDAzLCJpYXQiOjE2MTQ0MjM4MDMsImlzcyI6IkZpbGUgQnJvd3NlciJ9.BNpqWQIPIK6_nEFrIpAHYTTbfOhwex_QyWzfZkcf4a8"
}

#[get("/renew")]
fn renew() -> &'static str {
    login()
}

#[get("/", rank = 4)]
fn file(tmpl: State<String>) -> Html<String> {
    let mut tmpl_params: Context = Context::new();
    tmpl_params.insert("ReCaptcha", &false);
    tmpl_params.insert("ReCaptchaHost", "");
    tmpl_params.insert("Name", "FileBruster");
    tmpl_params.insert("StaticURL", "/static");
    tmpl_params.insert("Json", r#"{
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
          }"#);
    tmpl_params.insert("Theme", "dark");
    tmpl_params.insert("CSS", &false);
    Html(Tera::one_off(&tmpl, &tmpl_params, false).unwrap())
}

fn main() {
    let root = Box::new(std::env::current_dir().unwrap());
    let root: &'static Path = Box::leak(root);

    let file = String::from_utf8(std::fs::read("filebrowser/frontend/dist/index.html").unwrap()).unwrap();
    let tmpl = file.replace("[{[ if .", "{% if ")
                .replace("-]}]", "%}")
                .replace("[{[ end ]}]", "{% endif %}")
                .replace("[{[ else ]}]", "{% else %}")
                .replace("[{[ .", "{{ ")
                .replace("]}]", "}}");

    rocket::ignite()
        .manage(root)
        .manage(tmpl)
        .mount("/api", routes![login, renew, resources::get_resources, resources::get_resources_root])
        .mount("/static", StaticFiles::from("filebrowser/frontend/dist"))
        .mount("/", routes![file])
        .launch();
}
