pub mod invitation;

pub use invitation::*;

/// Configure routes for invitation-service
pub fn configure(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(
        actix_web::web::scope("/invitations")
            .route("", actix_web::web::post().to(create_invitation_handler))
            .route("/me", actix_web::web::get().to(list_my_invitations_handler))
            .route(
                "/validate",
                actix_web::web::post().to(validate_invitation_handler),
            )
            .route("/use", actix_web::web::post().to(use_invitation_handler))
            .route(
                "/{id}/uses",
                actix_web::web::get().to(get_invitation_uses_handler),
            )
            .route(
                "/{id}",
                actix_web::web::delete().to(revoke_invitation_handler),
            ),
    );
}
