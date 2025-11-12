use actix_web::{middleware::Logger, web, App, HttpServer};
use sqlx::postgres::PgPoolOptions;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    let config = user_service::config::Config::from_env().expect("Failed to load configuration");

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database.url)
        .await
        .expect("Failed to connect to database");

    let server_host = config.server.host.clone();
    let server_port = config.server.port;
    let allowed_origins = config.cors.allowed_origins.clone();

    tracing::info!("Starting User Service on {}:{}", server_host, server_port);

    HttpServer::new(move || {
        let mut cors = actix_cors::Cors::default();
        for origin in &allowed_origins {
            cors = cors.allowed_origin(origin);
        }
        let cors = cors
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "PATCH"])
            .allowed_headers(vec![
                actix_web::http::header::AUTHORIZATION,
                actix_web::http::header::ACCEPT,
                actix_web::http::header::CONTENT_TYPE,
            ])
            .max_age(3600);

        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            .app_data(web::Data::new(pool.clone()))
            .route("/health", web::get().to(user_service::handlers::health))
            // Profile endpoints
            .route(
                "/v1/profiles/{id}",
                web::get().to(user_service::handlers::get_profile),
            )
            .route(
                "/v1/profiles/{id}",
                web::put().to(user_service::handlers::update_profile),
            )
            // Profile links endpoints
            .route(
                "/v1/profiles/{id}/links",
                web::get().to(user_service::handlers::get_profile_links),
            )
            .route(
                "/v1/profiles/{id}/links",
                web::post().to(user_service::handlers::create_profile_link),
            )
            .route(
                "/v1/profiles/{id}/links/{link_id}",
                web::put().to(user_service::handlers::update_profile_link),
            )
            .route(
                "/v1/profiles/{id}/links/{link_id}",
                web::delete().to(user_service::handlers::delete_profile_link),
            )
            .route(
                "/v1/profiles/{id}/links/reorder",
                web::patch().to(user_service::handlers::reorder_profile_links),
            )
            // Language proficiency endpoints
            .route(
                "/v1/profiles/{id}/languages",
                web::get().to(user_service::handlers::get_language_proficiencies),
            )
            .route(
                "/v1/profiles/{id}/languages",
                web::post().to(user_service::handlers::create_language_proficiency),
            )
            .route(
                "/v1/profiles/{id}/languages/{lang_id}",
                web::put().to(user_service::handlers::update_language_proficiency),
            )
            .route(
                "/v1/profiles/{id}/languages/{lang_id}",
                web::delete().to(user_service::handlers::delete_language_proficiency),
            )
            // Settings endpoints
            .route(
                "/v1/users/{id}/settings",
                web::get().to(user_service::handlers::get_user_settings),
            )
            .route(
                "/v1/users/{id}/settings",
                web::put().to(user_service::handlers::update_user_settings),
            )
            .route(
                "/v1/users/{id}/settings/notifications",
                web::get().to(user_service::handlers::get_notification_settings),
            )
            .route(
                "/v1/users/{id}/settings/notifications",
                web::put().to(user_service::handlers::update_notification_settings),
            )
            // User connection endpoints
            .route(
                "/v1/users/{id}/connections/follow/{target_id}",
                web::post().to(user_service::handlers::follow_user),
            )
            .route(
                "/v1/users/{id}/connections/follow/{target_id}",
                web::delete().to(user_service::handlers::unfollow_user),
            )
            .route(
                "/v1/users/{id}/connections/followers",
                web::get().to(user_service::handlers::get_followers),
            )
            .route(
                "/v1/users/{id}/connections/following",
                web::get().to(user_service::handlers::get_following),
            )
            .route(
                "/v1/users/{id}/connections/block/{target_id}",
                web::post().to(user_service::handlers::block_user),
            )
            .route(
                "/v1/users/{id}/connections/block/{target_id}",
                web::delete().to(user_service::handlers::unblock_user),
            )
            .route(
                "/v1/users/{id}/connections/blocked",
                web::get().to(user_service::handlers::get_blocked_users),
            )
            // GDPR data export endpoints
            .route(
                "/v1/users/{id}/data/export",
                web::post().to(user_service::handlers::request_data_export),
            )
            .route(
                "/v1/users/{id}/data/export",
                web::get().to(user_service::handlers::list_data_exports),
            )
            .route(
                "/v1/users/{id}/data/export/{export_id}",
                web::get().to(user_service::handlers::download_data_export),
            )
            // TODO: GDPR account deletion endpoints (in progress)
            .service(user_service::openapi::swagger_ui())
    })
    .bind((server_host.as_str(), server_port))?
    .run()
    .await
}
