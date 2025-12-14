use serde::Deserialize;
use serde::{self, Serialize};
use std::collections::HashMap;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Deserialize, ToSchema)]
pub struct UploadQuery {
    #[serde(rename = "type")]
    pub upload_type: String, // "profile" or "cover"
}

#[derive(Serialize, ToSchema)]
pub struct UploadResponse {
    pub signed_upload_url: String,
    pub public_url: String,
}

#[derive(Deserialize, ToSchema)]
pub struct SlotQuery {
    #[schema(value_type = String, format = "date")]
    pub date: String, // Format should be "YYYY-MM-DD"
    pub service_id: Uuid,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct TimeSlot {
    pub start_time: String, // ISO 8601 / RFC 3339
    pub end_time: String,
}

#[derive(Serialize, ToSchema)]
pub struct FreeBusyRequest {
    #[serde(rename = "timeMin")]
    pub time_min: String,
    #[serde(rename = "timeMax")]
    pub time_max: String,
    pub items: Vec<FreeBusyRequestItem>,
}

#[derive(Serialize, ToSchema)]
pub struct FreeBusyRequestItem {
    pub id: String, // Calendar ID (usually "primary")
}

#[derive(Deserialize, Debug, ToSchema)]
pub struct FreeBusyResponse {
    pub calendars: HashMap<String, FreeBusyCalendar>,
}

#[derive(Deserialize, Debug, ToSchema)]
pub struct FreeBusyCalendar {
    pub busy: Vec<FreeBusyTime>,
}

#[derive(Deserialize, Debug, ToSchema)]
pub struct FreeBusyTime {
    pub start: String, // RFC3339 string
    pub end: String,   // RFC3339 string
}
