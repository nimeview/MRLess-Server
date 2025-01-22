mod user;
mod chat;
mod API;

use axum::{
    routing::{get, post},
    Router, Json,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tower_http::cors::{CorsLayer, Any};


#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_headers(Any)
        .allow_methods(Any)
        .allow_origin(Any);

    let app = Router::new()
        .route("/chat/userList", post(chat::user_list))
        .route("/user/update_data", post(user::update_user_data))
        .route("/user/register", post(user::register_user))
        .route("/user/login", post(user::login_user))
        .route("/", get(root))
        .layer(cors);

    let addr = SocketAddr::from(([0, 0, 0, 0], 1488));
    println!("Server running at http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Welcome to the Rust server!"
}