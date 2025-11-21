pub mod community;

use actix_web::web;
use community::{create_community, get_community, list_communities, assign_manager, revoke_manager};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("")
            .route(web::get().to(list_communities))
            .route(web::post().to(create_community))
    )
    .service(
        web::resource("/{id}")
            .route(web::get().to(get_community))
    )
    .service(
        web::resource("/{id}/manager")
            .route(web::post().to(assign_manager))
    )
    .service(
        web::resource("/{id}/manager/{user_id}")
            .route(web::delete().to(revoke_manager))
    );
}



