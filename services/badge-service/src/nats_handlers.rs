use crate::models::{CourseCompletedEvent, UserRegisteredEvent};
use crate::services::badge;
use futures_util::StreamExt;
use shared_lib::{Database, NatsClient};
use std::env;

/// Subscribe to user.registered events
/// In development mode, auto-grants Code of Conduct badge
pub async fn subscribe_user_registered(nats: NatsClient, db: Database) {
    let dev_auto_grant = env::var("DEV_AUTO_GRANT_CODE_OF_CONDUCT")
        .unwrap_or_else(|_| "false".to_string())
        .parse::<bool>()
        .unwrap_or(false);

    tracing::info!(
        "Subscribing to user.registered events (DEV_AUTO_GRANT_CODE_OF_CONDUCT={})",
        dev_auto_grant
    );

    let mut subscriber = match nats.subscribe("user.registered.*").await {
        Ok(sub) => sub,
        Err(e) => {
            tracing::error!("Failed to subscribe to user.registered: {}", e);
            return;
        }
    };

    tokio::spawn(async move {
        while let Some(msg) = subscriber.next().await {
            match serde_json::from_slice::<UserRegisteredEvent>(&msg.payload) {
                Ok(event) => {
                    tracing::info!(
                        "User registered: {} ({}), territory: {}",
                        event.username,
                        event.user_id,
                        event.territory_code
                    );

                    // Auto-grant Code of Conduct badge in development mode
                    if dev_auto_grant {
                        match badge::award_badge(
                            &db,
                            &nats,
                            event.user_id,
                            "code-of-conduct",
                            None,
                            Some("Auto-granted in development mode".to_string()),
                        )
                        .await
                        {
                            Ok(award_id) => {
                                tracing::info!(
                                    "Auto-granted Code of Conduct badge to {} (award_id: {})",
                                    event.user_id,
                                    award_id
                                );
                                // Event already published by award_badge()
                            }
                            Err(e) => {
                                tracing::error!(
                                    "Failed to auto-grant Code of Conduct badge to {}: {}",
                                    event.user_id,
                                    e
                                );
                            }
                        }
                    }
                }
                Err(e) => {
                    tracing::error!("Failed to parse user.registered event: {}", e);
                }
            }
        }
    });
}

/// Subscribe to course.completed events
/// Awards badges based on course completion
pub async fn subscribe_course_completed(nats: NatsClient, db: Database) {
    tracing::info!("Subscribing to course.completed events");

    let mut subscriber = match nats.subscribe("course.completed.*").await {
        Ok(sub) => sub,
        Err(e) => {
            tracing::error!("Failed to subscribe to course.completed: {}", e);
            return;
        }
    };

    tokio::spawn(async move {
        while let Some(msg) = subscriber.next().await {
            match serde_json::from_slice::<CourseCompletedEvent>(&msg.payload) {
                Ok(event) => {
                    tracing::info!(
                        "Course completed: {} by user {} (score: {:?})",
                        event.course_code,
                        event.user_id,
                        event.score
                    );

                    // Award Code of Conduct badge if course is code_of_conduct_training
                    if event.course_code == "code_of_conduct_training" {
                        if let Some(score) = event.score {
                            if score >= 80 {
                                match badge::award_badge(
                                    &db,
                                    &nats,
                                    event.user_id,
                                    "code-of-conduct",
                                    None,
                                    Some(format!(
                                        "Completed Code of Conduct course with score {}",
                                        score
                                    )),
                                )
                                .await
                                {
                                    Ok(award_id) => {
                                        tracing::info!(
                                            "Awarded Code of Conduct badge to {} (award_id: {})",
                                            event.user_id,
                                            award_id
                                        );
                                        // Event already published by award_badge()
                                    }
                                    Err(e) => {
                                        tracing::error!(
                                            "Failed to award Code of Conduct badge to {}: {}",
                                            event.user_id,
                                            e
                                        );
                                    }
                                }
                            } else {
                                tracing::warn!(
                                    "User {} completed Code of Conduct course but score {} is below 80",
                                    event.user_id,
                                    score
                                );
                            }
                        }
                    }
                }
                Err(e) => {
                    tracing::error!("Failed to parse course.completed event: {}", e);
                }
            }
        }
    });
}

/// Initialize all NATS subscriptions
pub async fn initialize_subscriptions(nats: NatsClient, db: Database) {
    subscribe_user_registered(nats.clone(), db.clone()).await;
    subscribe_course_completed(nats.clone(), db.clone()).await;
}
