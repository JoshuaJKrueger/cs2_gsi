use chrono::Local;
use serde_json::{Value, json};
use std::fs;
use uuid::Uuid;

pub struct PayloadLogger;

impl PayloadLogger {
    pub async fn log_payload(payload: &Value) -> Result<String, String> {
        let data_dir = "data";
        fs::create_dir_all(data_dir).map_err(|e| {
            eprintln!("Failed to create data directory: {}", e);
            format!("IO error: {}", e)
        })?;

        let now = Local::now();
        let timestamp = now.format("%Y-%m-%d_%H%M%S");
        let request_id = Uuid::new_v4();
        let filename = format!("{}/gsi_payload_{}_{}.json", data_dir, timestamp, request_id);
        let log_entry = json!({
            "request_id": request_id.to_string(),
            "timestamp": now.to_rfc3339(),
            "payload": payload
        });

        fs::write(&filename, log_entry.to_string()).map_err(|e| {
            eprintln!("Failed to write payload to file: {}", e);
            format!("IO error: {}", e)
        })?;

        Ok(filename)
    }
}
