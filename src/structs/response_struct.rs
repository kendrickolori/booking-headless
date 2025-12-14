use serde::Serialize;
use utoipa::ToSchema;

use crate::structs::db_struct::{AvailabilityRule, User};

#[derive(Serialize, Debug, ToSchema)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: Option<String>,
}

#[derive(Serialize, ToSchema)]
pub struct MergedUserProfile {
    pub profile: User,
    pub availability: Vec<AvailabilityRule>,
}

#[derive(Serialize, ToSchema)]
pub struct EmptyStruct {}
