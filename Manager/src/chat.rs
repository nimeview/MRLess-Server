use axum::Json;
use crate::API::{ApiResponse, User, connection};


pub async fn user_list(Json(payload): Json<User>) -> Json<ApiResponse> {
    let request = serde_json::json!({
        "action": "user_list",
        "username": payload.username,
    });
    let response = connection(request).await;
    Json(response)
}

pub async fn find_chats(Json(payload): Json<User>) -> Json<ApiResponse> {
    let request = serde_json::json!({
        "action": "find_chats",
        "username": payload.username,
    });
    let response = connection(request).await;
    Json(response)
}