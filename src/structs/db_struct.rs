use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateUser {
    pub username: String,
    pub business_name: String,
    pub email: String,
    pub location: Option<String>,
    pub phone_number: Option<String>,
    pub description: Option<String>,
    pub phone_number_is_whatsapp: Option<bool>,
}

#[derive(Serialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub business_name: String,
    pub email: String,
    pub location: Option<String>,
    pub phone_number: Option<String>,
    pub cover_image_url: Option<String>,
    pub profile_image_url: Option<String>,
    pub description: Option<String>,
    pub is_verified: Option<bool>,
    pub google_is_connected: Option<bool>,
    pub phone_number_is_whatsapp: Option<bool>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub last_login: Option<DateTime<Utc>>,
}
