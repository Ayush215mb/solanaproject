#![allow(unused)]
use std::net::SocketAddr;

use axum::{Router, response::Html, routing::get};

#[tokio::main]
async fn main() {
    // our router
    let app = Router::<()>::new()
        .route("/", get(root))
        .route("/foo", get(get_foo).post(post_foo))
        .route("/foo/bar", get(foo_bar));

    // which calls one of these handlers

    // Handlers
    async fn root() -> Html<&'static str> {
        Html("<h1>Welcome to the root!</h1>")
    }

    async fn get_foo() -> Html<&'static str> {
        Html("<h1>GET /foo</h1>")
    }

    async fn post_foo() -> Html<&'static str> {
        Html("<h1>POST /foo</h1>")
    }

    async fn foo_bar() -> Html<&'static str> {
        Html("<h1>GET /foo/bar</h1>")
    }

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
