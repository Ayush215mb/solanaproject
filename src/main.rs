use axum::{
    Json, Router,
    response::{Html, IntoResponse},
    routing::{get, post},
};
use serde::Serialize;
use solana_sdk::signature::{Keypair, Signer};
use std::net::SocketAddr;
#[tokio::main]
async fn main() {
    // Define app routes
    let app = Router::new()
        .route("/", get(root))
        .route("/keypair", get(get_keypair).post(post_keypair))
        .route("/token/create", get(token_create))
        .route("/token/mint", get(get_mint))
        .route("/message/sign", get(get_mint))
        .route("/message/verify", get(get_mint))
        .route("/send/sol", get(get_mint))
        .route("/send/token", get(get_mint));

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

// JSON response struct
#[derive(Serialize)]
struct KeypairResponse {
    success: bool,
    data: KeypairData,
}

#[derive(Serialize)]
struct KeypairData {
    pubkey: String,
    secret: String,
}

async fn root() -> Html<&'static str> {
    Html("<h1>Welcome to the root!</h1>")
}

async fn get_keypair() -> Html<&'static str> {
    Html("<h1>GET keypair</h1>")
}

async fn post_keypair() -> impl IntoResponse {
    let keypair = Keypair::new();
    let pubkey = keypair.pubkey().to_string();
    let secret = bs58::encode(keypair.to_bytes()).into_string();

    let response = KeypairResponse {
        success: true,
        data: KeypairData { pubkey, secret },
    };

    Json(response)
}

async fn token_create() -> Html<&'static str> {
    Html("<h1>token_create</h1>")
}
async fn get_mint() -> Html<&'static str> {
    Html("<h1>get mint</h1>")
}
