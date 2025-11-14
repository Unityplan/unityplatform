pub mod handlers;
pub mod models;
pub mod nats_handlers;
pub mod services;

// Re-export handlers for main.rs
pub use handlers::badge;
