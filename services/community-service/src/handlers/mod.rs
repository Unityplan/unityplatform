pub mod community;

use actix_web::web;
use community::{
    add_requirement, assign_manager, create_community, get_community, get_effective_managers,
    get_effective_requirements, get_group_summary, get_managed_communities, get_membership,
    join_community, leave_community, list_communities, list_requirements, remove_requirement,
    revoke_manager, update_community,
};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("")
            .route(web::get().to(list_communities))
            .route(web::post().to(create_community)),
    )
    .service(web::resource("/managed-by-me").route(web::get().to(get_managed_communities)))
    .service(
        web::resource("/{id}")
            .route(web::get().to(get_community))
            .route(web::put().to(update_community)),
    )
    .service(web::resource("/{id}/join").route(web::post().to(join_community)))
    .service(web::resource("/{id}/leave").route(web::post().to(leave_community)))
    .service(web::resource("/{id}/membership").route(web::get().to(get_membership)))
    .service(web::resource("/{id}/group-summary").route(web::get().to(get_group_summary)))
    .service(web::resource("/{id}/effective-requirements").route(web::get().to(get_effective_requirements)))
    .service(web::resource("/{id}/managers").route(web::get().to(get_effective_managers)))
    .service(web::resource("/{id}/manager").route(web::post().to(assign_manager)))
    .service(web::resource("/{id}/manager/{user_id}").route(web::delete().to(revoke_manager)))
    .service(
        web::resource("/{id}/requirements")
            .route(web::post().to(add_requirement))
            .route(web::get().to(list_requirements)),
    )
    .service(
        web::resource("/{id}/requirements/{badge_id}").route(web::delete().to(remove_requirement)),
    );
}
