use axum::{Json, http::StatusCode, response::IntoResponse};

use crate::logger::PayloadLogger;
use serde_json::Value;

pub async fn handle_gsi_post(Json(payload): Json<Value>) -> impl IntoResponse {
    match PayloadLogger::log_payload(&payload).await {
        Ok(filename) => {
            println!("[GSI] Received and logged payload to: {}", filename);
            (StatusCode::OK, "OK")
        }
        Err(e) => {
            eprintln!("[GSI] Failed to log payload: {}", e);
            (StatusCode::OK, "OK")
        }
    }
}
