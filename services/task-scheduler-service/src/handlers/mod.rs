pub mod cleanup;

pub use cleanup::*;

/// Configure routes for task-scheduler-service
pub fn configure(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(
        actix_web::web::scope("/cleanup")
            .route("/users", actix_web::web::post().to(trigger_cleanup))
            .route("/stats", actix_web::web::get().to(get_cleanup_stats)),
    );
}
