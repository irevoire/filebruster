#![feature(proc_macro_hygiene, decl_macro)]

mod json;
mod r#static;
mod resources;

#[macro_use]
extern crate rocket;

use rocket_contrib::serve::StaticFiles;
use std::path::Path;

#[post("/login")]
fn login() -> &'static str {
    "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VyIjp7ImlkIjoxLCJsb2NhbGUiOiJlbiIsInZpZXdNb2RlIjoibW9zYWljIiwic2luZ2xlQ2xpY2siOmZhbHNlLCJwZXJtIjp7ImFkbWluIjpmYWxzZSwiZXhlY3V0ZSI6dHJ1ZSwiY3JlYXRlIjpmYWxzZSwicmVuYW1lIjpmYWxzZSwibW9kaWZ5IjpmYWxzZSwiZGVsZXRlIjpmYWxzZSwic2hhcmUiOnRydWUsImRvd25sb2FkIjp0cnVlfSwiY29tbWFuZHMiOltdLCJsb2NrUGFzc3dvcmQiOmZhbHNlLCJoaWRlRG90ZmlsZXMiOmZhbHNlfSwiZXhwIjoxNjE0NDMxMDAzLCJpYXQiOjE2MTQ0MjM4MDMsImlzcyI6IkZpbGUgQnJvd3NlciJ9.BNpqWQIPIK6_nEFrIpAHYTTbfOhwex_QyWzfZkcf4a8"
}

#[get("/renew")]
fn renew() -> &'static str {
    login()
}


fn main() {
    let root = Box::new(std::env::current_dir().unwrap());
    let root: &'static Path = Box::leak(root);

    let file =r#static::Static::get("index.html").unwrap();
    let file = String::from_utf8(file.to_vec()).unwrap();

    let tmpl = file
        .replace("[{[ if .", "{% if ")
        .replace("-]}]", "%}")
        .replace("[{[ end ]}]", "{% endif %}")
        .replace("[{[ else ]}]", "{% else %}")
        .replace("[{[ .", "{{ ")
        .replace("]}]", "}}");

    rocket::ignite()
        .manage(root)
        .manage(tmpl)
        .mount(
            "/api",
            routes![
                login,
                renew,
                resources::get_resources,
                resources::get_resources_root
            ],
        )
        .mount("/static", StaticFiles::from("filebrowser/frontend/dist"))
        .mount("/", routes![r#static::file])
        .launch();
}
