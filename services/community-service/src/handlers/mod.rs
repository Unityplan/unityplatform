pub mod community;

use actix_web::web;
use community::{
    add_requirement, assign_manager, create_community, get_children, get_community,
    get_community_context, get_effective_managers, get_effective_requirements, get_geo_markers,
    get_group_summary, get_hierarchy, get_managed_communities, get_membership, get_root_communities,
    join_community, leave_community, list_communities, list_communities_paginated, list_requirements,
    remove_requirement, revoke_manager, update_community,
};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg
    // Static routes MUST come before parameterized routes
    .service(web::resource("/roots").route(web::get().to(get_root_communities)))
    .service(web::resource("/hierarchy").route(web::get().to(get_hierarchy)))
    .service(web::resource("/geo-markers").route(web::get().to(get_geo_markers)))
    .service(web::resource("/paginated").route(web::get().to(list_communities_paginated)))
    .service(web::resource("/managed-by-me").route(web::get().to(get_managed_communities)))
    // Collection routes
    .service(
        web::resource("")
            .route(web::get().to(list_communities))
            .route(web::post().to(create_community)),
    )
    // Parameterized routes (must come after static routes)
    .service(
        web::resource("/{id}")
            .route(web::get().to(get_community))
            .route(web::put().to(update_community)),
    )
    .service(web::resource("/{id}/children").route(web::get().to(get_children)))
    .service(web::resource("/{id}/context").route(web::get().to(get_community_context)))
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
