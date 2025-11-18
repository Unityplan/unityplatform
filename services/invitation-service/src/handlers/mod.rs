pub mod invitation;

pub use invitation::*;

/// Configure routes for invitation-service
pub fn configure(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(
        actix_web::web::scope("/invitations")
            .route("/validate", actix_web::web::post().to(validate_invitation_handler)),
    );
}
