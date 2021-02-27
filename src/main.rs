#![feature(proc_macro_hygiene, decl_macro)]

mod json;
mod resources;

#[macro_use]
extern crate rocket;

#[get("/login")]
fn login() -> &'static str {
    "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VyIjp7ImlkIjoxLCJsb2NhbGUiOiJlbiIsInZpZXdNb2RlIjoibW9zYWljIiwic2luZ2xlQ2xpY2siOmZhbHNlLCJwZXJtIjp7ImFkbWluIjpmYWxzZSwiZXhlY3V0ZSI6dHJ1ZSwiY3JlYXRlIjpmYWxzZSwicmVuYW1lIjpmYWxzZSwibW9kaWZ5IjpmYWxzZSwiZGVsZXRlIjpmYWxzZSwic2hhcmUiOnRydWUsImRvd25sb2FkIjp0cnVlfSwiY29tbWFuZHMiOltdLCJsb2NrUGFzc3dvcmQiOmZhbHNlLCJoaWRlRG90ZmlsZXMiOmZhbHNlfSwiZXhwIjoxNjE0NDMxMDAzLCJpYXQiOjE2MTQ0MjM4MDMsImlzcyI6IkZpbGUgQnJvd3NlciJ9.BNpqWQIPIK6_nEFrIpAHYTTbfOhwex_QyWzfZkcf4a8"
}

#[get("/renew")]
fn renew() -> &'static str {
    login()
}

use std::path::Path;

fn main() {
    let root = Box::new(std::env::current_dir().unwrap());
    let root: &'static Path = Box::leak(root);

    rocket::ignite()
        .manage(root)
        .mount("/api", routes![login, renew, resources::get_resources])
        .launch();
}
