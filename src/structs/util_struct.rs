use serde::Deserialize;
use serde::{self, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct UploadQuery {
    #[serde(rename = "type")]
    pub upload_type: String, // "profile" or "cover"
}

#[derive(Serialize)]
pub struct UploadResponse {
    pub signed_upload_url: String,
    pub public_url: String,
}

#[derive(Deserialize)]
pub struct SlotQuery {
    pub date: String, // Format should be "YYYY-MM-DD"
    pub service_id: Uuid,
}

#[derive(Serialize)]
pub struct TimeSlot {
    pub start_time: String, // ISO 8601 / RFC 3339
    pub end_time: String,
}
