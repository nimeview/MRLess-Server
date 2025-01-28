use axum::Json;
use serde::Deserialize;
use crate::{api::{Response, connection, ApiResponse}};

pub async fn login_user(Json(payload): Json<Response>) -> Json<ApiResponse> {
    let request = serde_json::json!({
        "action": "login",
        "username": payload.username,
        "password": payload.password,
    });
    let response = connection(request).await;
    Json(response)
}

pub async fn register_user(Json(payload): Json<Response>) -> Json<ApiResponse> {
    let request = serde_json::json!({
        "action": "register",
        "username": payload.username,
        "password": payload.password,
        "email": payload.email,
    });
    let response = connection(request).await;
    Json(response)
}

pub async fn update_user_data(Json(payload): Json<Response>) -> Json<ApiResponse> {
    let request = serde_json::json!({
        "action": "update_data",
        "username": payload.username,
        "option": payload.option,
        "data": payload.data,
    });
    let response = connection(request).await;
    Json(response)
}