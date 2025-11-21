use actix_web::{web, HttpResponse};
use shared_lib::{Result, ValidatedJson, AuthUser, NatsClient};
use uuid::Uuid;
use crate::models::{CreateCommunityRequest, CommunityFilter, AssignManagerRequest};
use crate::services::CommunityService;


pub async fn create_community(
    service: web::Data<CommunityService>,
    auth_user: AuthUser,
    body: ValidatedJson<CreateCommunityRequest>,
) -> Result<HttpResponse> {
    let community = service.create_community(body.into_inner(), auth_user.id).await?;
    Ok(HttpResponse::Created().json(community))
}

pub async fn get_community(
    service: web::Data<CommunityService>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse> {
    let community = service.get_community(path.into_inner()).await?;
    Ok(HttpResponse::Ok().json(community))
}

pub async fn list_communities(
    service: web::Data<CommunityService>,
    query: web::Query<CommunityFilter>,
) -> Result<HttpResponse> {
    let communities = service.list_communities(query.into_inner()).await?;
    Ok(HttpResponse::Ok().json(communities))
}

pub async fn assign_manager(
    service: web::Data<CommunityService>,
    nats: web::Data<NatsClient>,
    auth_user: AuthUser,
    path: web::Path<Uuid>,
    body: ValidatedJson<AssignManagerRequest>,
) -> Result<HttpResponse> {
    let community_id = path.into_inner();
    service.assign_manager(community_id, body.user_id, auth_user.id, &nats).await?;
    Ok(HttpResponse::Ok().finish())
}

pub async fn revoke_manager(
    service: web::Data<CommunityService>,
    nats: web::Data<NatsClient>,
    auth_user: AuthUser,
    path: web::Path<(Uuid, Uuid)>, // community_id, user_id
) -> Result<HttpResponse> {
    let (community_id, user_id) = path.into_inner();
    service.revoke_manager(community_id, user_id, auth_user.id, &nats).await?;
    Ok(HttpResponse::Ok().finish())
}


