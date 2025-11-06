use crate::structs::db_struct::TokenClaims;
use crate::structs::response_struct::ApiResponse;
use actix_web::HttpResponse;
use actix_web::error::ResponseError;
use actix_web::http::StatusCode;
use actix_web::{FromRequest, HttpRequest, dev::Payload};
use jsonwebtoken::{DecodingKey, Validation, decode};
use std::env;
use std::fmt::{self, Display, Formatter};
use std::future::{Ready, ready};
use uuid::Uuid;

#[derive(Debug)]
pub struct ApiAuthError {
    response: ApiResponse<()>,
    status_code: StatusCode,
}

impl Display for ApiAuthError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.response
                .message
                .as_deref()
                .unwrap_or("Authentication Error")
        )
    }
}

// This tells Actix how to convert our
// ApiAuthError into a real HttpResponse.
impl ResponseError for ApiAuthError {
    fn status_code(&self) -> StatusCode {
        self.status_code
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code).json(&self.response)
    }
}

fn create_401_error(message: &str) -> ApiAuthError {
    ApiAuthError {
        response: ApiResponse::<()> {
            success: false,
            data: None,
            message: Some(message.to_string()),
        },
        status_code: StatusCode::UNAUTHORIZED,
    }
}

fn create_500_error(message: &str) -> ApiAuthError {
    ApiAuthError {
        response: ApiResponse::<()> {
            success: false,
            data: None,
            message: Some(message.to_string()),
        },
        status_code: StatusCode::INTERNAL_SERVER_ERROR,
    }
}

pub struct AuthenticatedUser {
    pub user_id: Uuid,
}

impl FromRequest for AuthenticatedUser {
    type Error = ApiAuthError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let auth_header = req.headers().get("Authorization");

        if auth_header.is_none() {
            return ready(Err(create_401_error("Missing authentication token.")));
        }

        let auth_str = auth_header.unwrap().to_str().unwrap_or("");
        if !auth_str.starts_with("Bearer ") {
            return ready(Err(create_401_error("Invalid token format.")));
        }

        let token = auth_str[7..].to_string();

        let jwt_secret = match env::var("JWT_SECRET") {
            Ok(secret) => secret,

            Err(e) => {
                return ready(Err(create_500_error(&e.to_string())));
            }
        };

        let token_data = decode::<TokenClaims>(
            &token,
            &DecodingKey::from_secret(jwt_secret.as_ref()),
            &Validation::default(),
        );

        match token_data {
            Ok(token) => ready(Ok(AuthenticatedUser {
                user_id: token.claims.sub,
            })),

            Err(_) => ready(Err(create_401_error("Invalid or expired token."))),
        }
    }
}
