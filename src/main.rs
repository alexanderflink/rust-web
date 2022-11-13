use axum::{response::Html, routing::get, Router};
use std::fs;
use std::net::SocketAddr;
use sycamore::prelude::*;

#[tokio::main]
async fn main() {
    let app: Router = Router::new().route("/", get(root));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> Html<String> {
    let document_html = String::from_utf8(fs::read("index.html").expect("index.html should exist"))
        .expect("index.html should be valid utf-8");

    let html = sycamore::render_to_string(|cx| {
        view! {cx,
            h1 {
           "Hello, world!"
            }
        }
    });

    let result = document_html.replace("%sycamore", &html);

    Html(result)
}
