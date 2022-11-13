use axum::{
    routing::{get, post},
    Router,
};

mod json;
mod resources;
mod r#static;

async fn login() -> &'static str {
    "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VyIjp7ImlkIjoxLCJsb2NhbGUiOiJlbiIsInZpZXdNb2RlIjoibW9zYWljIiwic2luZ2xlQ2xpY2siOmZhbHNlLCJwZXJtIjp7ImFkbWluIjpmYWxzZSwiZXhlY3V0ZSI6dHJ1ZSwiY3JlYXRlIjpmYWxzZSwicmVuYW1lIjpmYWxzZSwibW9kaWZ5IjpmYWxzZSwiZGVsZXRlIjpmYWxzZSwic2hhcmUiOnRydWUsImRvd25sb2FkIjp0cnVlfSwiY29tbWFuZHMiOltdLCJsb2NrUGFzc3dvcmQiOmZhbHNlLCJoaWRlRG90ZmlsZXMiOmZhbHNlfSwiZXhwIjoxNjE0NDMxMDAzLCJpYXQiOjE2MTQ0MjM4MDMsImlzcyI6IkZpbGUgQnJvd3NlciJ9.BNpqWQIPIK6_nEFrIpAHYTTbfOhwex_QyWzfZkcf4a8"
}

async fn renew() -> &'static str {
    login().await
}

#[tokio::main]
async fn main() {
    let root = std::env::current_dir().unwrap();

    let file = r#static::Static::get("index.html").expect("index.html");
    let file = String::from_utf8(file.to_vec()).unwrap();

    let tmpl = file
        .replace("[{[ if .", "{% if ")
        .replace("-]}]", "%}")
        .replace("[{[ end ]}]", "{% endif %}")
        .replace("[{[ else ]}]", "{% else %}")
        .replace("[{[ .", "{{ ")
        .replace("]}]", "}}");

    let app = Router::new()
        .route("/login", post(login))
        .route("/renew", get(renew))
        .nest("/resources", resources::setup_router(root))
        .nest("/", r#static::setup_router(tmpl));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
