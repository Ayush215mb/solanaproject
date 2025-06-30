use axum::{Router, response::Html, routing::get};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // Define app routes
    let app = Router::new()
        .route("/", get(root))
        .route("/keypair", get(get_foo).post(post_foo))
        .route("/token/create", get(foo_bar));

    // Get port from Railway's environment variable
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()
        .expect("PORT must be a number");

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("Server running on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

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
