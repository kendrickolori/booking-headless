use crate::{
    routes::utils_routes::internal_server_error_response,
    structs::{
        db_struct::{CreateUser, User},
        response_struct::ApiResponse,
    },
};
use actix_web::{HttpResponse, Responder, web};
use sqlx::PgPool;
use uuid::Uuid;

/* -------------------------------------------------------------------------- */
/*                                      -                                     */
/* -------------------------------------------------------------------------- */

async fn create_user(pool: web::Data<PgPool>, body: web::Json<CreateUser>) -> impl Responder {
    let new_user = body.into_inner();

    match sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (
            username, business_name, email, location, phone_number, 
            description, phone_number_is_whatsapp
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING *
        "#,
        new_user.username,
        new_user.business_name,
        new_user.email,
        new_user.location,
        new_user.phone_number,
        new_user.description,
        new_user.phone_number_is_whatsapp.unwrap_or(false)
    )
    .fetch_one(pool.get_ref())
    .await
    {
        Ok(created_user) => HttpResponse::Created().json(ApiResponse {
            message: Some("User created successfully".to_string()),
            data: Some(created_user),
            success: true,
        }),

        Err(sqlx::Error::Database(db_err)) => {
            if db_err.is_unique_violation() {
                HttpResponse::Conflict().json(ApiResponse::<()> {
                    message: Some("Username or email already exists".to_string()),
                    data: None,
                    success: false,
                })
            } else {
                HttpResponse::InternalServerError().json(ApiResponse::<()> {
                    message: Some(db_err.to_string()),
                    data: None,
                    success: false,
                })
            }
        }

        Err(e) => internal_server_error_response(e.to_string()),
    }
}

/* -------------------------------------------------------------------------- */
/*                                      -                                     */
/* -------------------------------------------------------------------------- */
/* -------------------------------------------------------------------------- */
/*                                      -                                     */
/* -------------------------------------------------------------------------- */
/* -------------------------------------------------------------------------- */
/*                                      -                                     */
/* -------------------------------------------------------------------------- */

async fn get_user_by_id(path: web::Path<Uuid>, pool: web::Data<PgPool>) -> impl Responder {
    let user_id = path.into_inner();

    match sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", user_id)
        .fetch_one(pool.get_ref())
        .await
    {
        Ok(user) => HttpResponse::Ok().json(ApiResponse {
            message: Some("User retrieved successfully".to_string()),
            data: Some(user),
            success: true,
        }),

        Err(sqlx::Error::RowNotFound) => HttpResponse::NotFound().json(ApiResponse::<()> {
            message: Some("User not found".to_string()),
            data: None,
            success: false,
        }),

        Err(e) => internal_server_error_response(e.to_string()),
    }
}

/* -------------------------------------------------------------------------- */
/*                                      -                                     */
/* -------------------------------------------------------------------------- */
/* -------------------------------------------------------------------------- */
/*                                      -                                     */
/* -------------------------------------------------------------------------- */
/* -------------------------------------------------------------------------- */
/*                                      -                                     */
/* -------------------------------------------------------------------------- */

pub fn user_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .route("/{id}", web::get().to(get_user_by_id))
            .route("", web::post().to(create_user)),
    );
}
