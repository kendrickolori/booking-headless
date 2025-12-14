use utoipa::OpenApi;
use crate::routes::{appointment_routes, user_routes, service_routes};
use crate::structs::{db_struct, response_struct, util_struct};

#[derive(OpenApi)]
#[openapi(
    paths(
        // Appointments
        appointment_routes::create_appointment,
        appointment_routes::get_all_appointments,
        appointment_routes::get_appointment_by_id,
        
        // Services
        service_routes::create_service,
        service_routes::get_all_services,
        service_routes::get_service_by_id,
        // service_routes::update_service, // Add these if you annotated them
        // service_routes::delete_service,

        // Users
        user_routes::get_available_slots,
        // user_routes::get_me, // Add these if you annotated them
    ),
    components(
        schemas(
            // Request/Response bodies
            db_struct::CreateAppointment,
            db_struct::Appointment,
            db_struct::Service,
            db_struct::CreateService,
            db_struct::UpdateService,
            util_struct::TimeSlot,
            
            // Generic wrappers (Aliased for documentation)
            response_struct::ApiResponse<db_struct::Appointment>,
            response_struct::ApiResponse<Vec<db_struct::Service>>,
            response_struct::ApiResponse<Vec<util_struct::TimeSlot>>,
        )
    ),
    tags(
        (name = "Appointments", description = "Booking management"),
        (name = "Services", description = "Service catalog management"),
        (name = "Users", description = "User profile and availability")
    )
)]
pub struct ApiDoc;