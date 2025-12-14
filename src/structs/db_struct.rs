use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use time::{OffsetDateTime, Time};
use utoipa::ToSchema;
use uuid::Uuid;

time::serde::format_description!(time_format, Time, "[hour]:[minute]:[second]");

/* -------------------------------------------------------------------------- */
/*                                      -                                     */
/* -------------------------------------------------------------------------- */
/* -------------------------------------------------------------------------- */
/*                                    AUTH                                    */
/* -------------------------------------------------------------------------- */
/* -------------------------------------------------------------------------- */
/*                                      -                                     */
/* -------------------------------------------------------------------------- */
#[derive(Serialize, FromRow, ToSchema)]
pub struct Auth {
    pub id: Uuid,
    pub user_id: Uuid,
    pub google_id: String,

    #[serde(skip)]
    pub refresh_token: Option<String>,

    #[serde(with = "time::serde::rfc3339::option")]
    pub created_at: Option<OffsetDateTime>,

    #[serde(with = "time::serde::rfc3339::option")]
    pub updated_at: Option<OffsetDateTime>,
}

#[derive(Deserialize, ToSchema)]
pub struct GoogleCode {
    pub code: String,
}

#[derive(Deserialize, ToSchema)]
pub struct GoogleUserInfo {
    pub sub: String, // The unique Google ID (provider_id)
    pub email: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct TokenClaims {
    pub sub: Uuid, // The user.id
    pub exp: i64,
}

/* -------------------------------------------------------------------------- */
/*                                      -                                     */
/* -------------------------------------------------------------------------- */
/* -------------------------------------------------------------------------- */
/*                                    USER                                    */
/* -------------------------------------------------------------------------- */
/* -------------------------------------------------------------------------- */
/*                                      -                                     */
/* -------------------------------------------------------------------------- */

#[derive(Serialize, FromRow, ToSchema)]
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
    pub is_active: Option<bool>,

    #[serde(with = "time::serde::rfc3339::option")]
    pub created_at: Option<OffsetDateTime>,

    #[serde(with = "time::serde::rfc3339::option")]
    pub updated_at: Option<OffsetDateTime>,

    #[serde(with = "time::serde::rfc3339::option")]
    pub last_login: Option<OffsetDateTime>,
}

#[derive(Deserialize, ToSchema)]
pub struct UpdateUser {
    pub username: Option<String>,
    pub business_name: Option<String>,
    pub email: Option<String>,
    pub location: Option<String>,
    pub phone_number: Option<String>,
    pub description: Option<String>,
    pub phone_number_is_whatsapp: Option<bool>,
}

#[derive(Deserialize, ToSchema)]
pub struct UserStatus {
    pub status: Option<bool>,
}

/* -------------------------------------------------------------------------- */
/*                                      -                                     */
/* -------------------------------------------------------------------------- */
/* -------------------------------------------------------------------------- */
/*                                  SERVICES                                  */
/* -------------------------------------------------------------------------- */
/* -------------------------------------------------------------------------- */
/*                                      -                                     */
/* -------------------------------------------------------------------------- */

#[derive(Serialize, FromRow, ToSchema)]
pub struct Service {
    pub id: Uuid,
    pub user_id: Uuid,
    pub service_name: String,
    pub description: Option<String>,
    pub price: Option<Decimal>,
    pub duration_minutes: Option<i32>,
    pub image_url: Option<String>,
    pub category: Option<String>,

    #[serde(with = "time::serde::rfc3339::option")]
    pub created_at: Option<OffsetDateTime>,

    #[serde(with = "time::serde::rfc3339::option")]
    pub updated_at: Option<OffsetDateTime>,
}

#[derive(Deserialize, ToSchema)]
pub struct CreateService {
    pub service_name: String,
    pub description: Option<String>,
    pub price: Option<Decimal>,
    pub duration_minutes: Option<i32>,
    pub category: Option<String>,
}

#[derive(Deserialize, ToSchema)]
pub struct UpdateService {
    pub service_name: Option<String>,
    pub description: Option<String>,
    pub price: Option<Decimal>,
    pub duration_minutes: Option<i32>,
    pub category: Option<String>,
}

#[derive(Serialize, FromRow, ToSchema)]
pub struct UserWithServices {
    pub user: User,
    pub services: Vec<Service>,
}

/* -------------------------------------------------------------------------- */
/*                                      -                                     */
/* -------------------------------------------------------------------------- */
/* -------------------------------------------------------------------------- */
/*                                APPOINTMENTS                                */
/* -------------------------------------------------------------------------- */
/* -------------------------------------------------------------------------- */
/*                                      -                                     */
/* -------------------------------------------------------------------------- */

#[derive(Serialize, FromRow, ToSchema)]
pub struct Appointment {
    pub id: Uuid,
    pub service_id: Uuid,
    pub business_id: Uuid,
    pub customer_name: String,
    pub customer_email: Option<String>,
    pub customer_phone: Option<String>,
    pub notes: Option<String>,

    #[serde(with = "time::serde::rfc3339")]
    pub appointment_start_time: OffsetDateTime,

    #[serde(with = "time::serde::rfc3339")]
    pub appointment_end_time: OffsetDateTime,

    #[serde(with = "time::serde::rfc3339::option")]
    pub created_at: Option<OffsetDateTime>,

    #[serde(with = "time::serde::rfc3339::option")]
    pub updated_at: Option<OffsetDateTime>,
}

#[derive(Deserialize, ToSchema)]
pub struct CreateAppointment {
    pub service_id: Uuid,
    pub business_id: Uuid,
    pub customer_name: String,
    pub customer_email: Option<String>,
    pub customer_phone: Option<String>,
    pub notes: Option<String>,

    #[serde(with = "time::serde::rfc3339")]
    pub appointment_start_time: OffsetDateTime,
}

#[derive(Serialize, ToSchema)]
pub struct GoogleCalendarEvent {
    pub summary: String,
    pub description: String,
    pub start: GoogleEventDateTime,
    pub end: GoogleEventDateTime,
    pub attendees: Vec<GoogleEventAttendee>,
}

#[derive(Serialize, ToSchema)]
pub struct GoogleEventDateTime {
    #[serde(rename = "dateTime")]
    pub date_time: String,

    #[serde(rename = "timeZone")]
    pub time_zone: String,
}

#[derive(Serialize, ToSchema)]
pub struct GoogleEventAttendee {
    pub email: String,
}

/* -------------------------------------------------------------------------- */
/*                                      -                                     */
/* -------------------------------------------------------------------------- */
/* -------------------------------------------------------------------------- */
/*                              AVAILABILITY RULE                             */
/* -------------------------------------------------------------------------- */
/* -------------------------------------------------------------------------- */
/*                                      -                                     */
/* -------------------------------------------------------------------------- */

#[derive(Serialize, FromRow, ToSchema)]
pub struct AvailabilityRule {
    pub id: Uuid,
    pub user_id: Uuid,
    pub day_of_week: i32,
    pub time_zone: String,

    #[serde(with = "time_format")]
    #[schema(value_type = String, format = "date-time")]
    pub open_time: Time,

    #[serde(with = "time_format")]
    #[schema(value_type = String, format = "date-time")]
    pub close_time: Time,

    #[serde(with = "time::serde::rfc3339::option")]
    pub created_at: Option<OffsetDateTime>,

    #[serde(with = "time::serde::rfc3339::option")]
    pub updated_at: Option<OffsetDateTime>,
}

#[derive(Deserialize, ToSchema)]
pub struct SetAvailability {
    #[serde(rename = "slots")]
    pub rules: Vec<DayTimeSlot>,
}

#[derive(Deserialize, ToSchema)]
pub struct DayTimeSlot {
    pub day_of_week: i32,
    pub open_time: String,
    pub close_time: String,
    pub time_zone: String,
}
