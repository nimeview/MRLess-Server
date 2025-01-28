use serde::{Deserialize, Serialize};
use serde_json::Value;
use zmq::Context;
const ENDPOINT: &str = "tcp://127.0.0.1:25378";

#[derive(Deserialize)]
pub struct Response {
    pub username: Option<String>,

    pub password: Option<String>,
    pub email: Option<String>,

    pub option: Option<String>,
    pub data: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponse {
    pub success: bool,
    pub message: String,
}

pub async fn connection(request: Value) -> ApiResponse {
    let context = Context::new();
    let socket = context.socket(zmq::REQ).unwrap();

    socket.connect(ENDPOINT).unwrap();
    socket.send(&serde_json::to_string(&request).unwrap(), 0).unwrap();

    let response = socket.recv_string(0).unwrap().unwrap();
    let api_response: ApiResponse = serde_json::from_str(&response).unwrap();

    api_response
}