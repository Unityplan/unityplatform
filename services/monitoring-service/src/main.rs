use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer};
use bollard::Docker;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{info, warn};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceStats {
    pub cpu_usage_percent: Option<f64>,
    pub memory_usage_mb: Option<f64>,
    pub memory_limit_mb: Option<f64>,
    pub memory_percent: Option<f64>,
    pub network_rx_mb: Option<f64>,
    pub network_tx_mb: Option<f64>,
    pub block_read_mb: Option<f64>,
    pub block_write_mb: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceStatus {
    pub name: String,
    pub status: String, // "operational", "down", "not-deployed"
    pub container_state: Option<String>,
    pub health: Option<String>,
    pub uptime: Option<String>,
    pub resources: Option<ResourceStats>,
}

#[derive(Debug, Serialize)]
pub struct ServicesResponse {
    pub services: Vec<ServiceStatus>,
    pub timestamp: i64,
}

/// Get status of all monitored services
async fn get_services_status(docker: web::Data<Docker>) -> HttpResponse {
    let mut services = Vec::new();

    // Define services to monitor (from the dashboard config)
    let monitored_containers = vec![
        ("service-postgres-dk", "PostgreSQL"),
        ("service-redis-dk", "Redis"),
        ("service-nats-dk", "NATS"),
        ("service-ipfs-dk", "IPFS"),
        ("monitoring-grafana", "Grafana"),
        ("monitoring-prometheus", "Prometheus"),
        ("monitoring-jaeger", "Jaeger"),
        ("monitoring-postgres-exporter-dk", "PostgreSQL Exporter"),
        ("monitoring-redis-exporter-dk", "Redis Exporter"),
        ("monitoring-nats-exporter-dk", "NATS Exporter"),
        ("monitoring-node-exporter-dk", "Node Exporter"),
        ("monitoring-cadvisor-dk", "cAdvisor"),
        ("dev-adminer", "Adminer"),
        ("dev-redis-commander", "Redis Commander"),
        ("dev-forgejo", "Forgejo"),
        ("dev-registry", "Docker Registry"),
        ("dev-mailhog", "MailHog"),
        ("reverse-proxy-traefik", "Traefik"),
    ];

    // Get all containers
    match docker.list_containers::<String>(None).await {
        Ok(containers) => {
            let mut container_map: HashMap<String, bollard::models::ContainerSummary> =
                HashMap::new();

            for container in containers {
                if let Some(names) = &container.names {
                    if let Some(name) = names.first() {
                        let key = name.trim_start_matches('/').to_string();
                        container_map.insert(key, container);
                    }
                }
            }

            for (container_name, display_name) in monitored_containers {
                let status = if let Some(container) = container_map.get(container_name) {
                    let state = container.state.as_deref().unwrap_or("unknown");
                    let health = container.status.as_deref();

                    // Get resource stats if container is running
                    let resources = if state == "running" {
                        get_container_stats(&docker, container_name).await
                    } else {
                        None
                    };

                    ServiceStatus {
                        name: display_name.to_string(),
                        status: if state == "running" {
                            "operational"
                        } else {
                            "down"
                        }
                        .to_string(),
                        container_state: Some(state.to_string()),
                        health: health.map(|h| h.to_string()),
                        uptime: None, // Could calculate from created timestamp
                        resources,
                    }
                } else {
                    ServiceStatus {
                        name: display_name.to_string(),
                        status: "not-deployed".to_string(),
                        container_state: None,
                        health: None,
                        uptime: None,
                        resources: None,
                    }
                };

                services.push(status);
            }
        }
        Err(e) => {
            warn!("Failed to list containers: {}", e);
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to connect to Docker"
            }));
        }
    }

    // Add non-containerized services (running as processes)
    services.push(check_http_service("Frontend", "http://localhost:5173").await);
    services.push(check_http_service("Auth Service", "http://localhost:8001/api/v1/health").await);
    services.push(check_http_service("User Service", "http://localhost:8002/api/v1/health").await);

    HttpResponse::Ok().json(ServicesResponse {
        services,
        timestamp: chrono::Utc::now().timestamp(),
    })
}

/// Get container resource stats
async fn get_container_stats(docker: &Docker, container_name: &str) -> Option<ResourceStats> {
    use bollard::container::StatsOptions;
    use futures_util::StreamExt;

    let options = StatsOptions {
        stream: false,
        one_shot: true,
    };

    let mut stats_stream = docker.stats(container_name, Some(options));

    if let Some(Ok(stats)) = stats_stream.next().await {
        // Calculate CPU usage percentage
        let cpu_usage = {
            let cpu_delta = stats.cpu_stats.cpu_usage.total_usage as f64
                - stats.precpu_stats.cpu_usage.total_usage as f64;
            let system_delta = stats.cpu_stats.system_cpu_usage.unwrap_or(0) as f64
                - stats.precpu_stats.system_cpu_usage.unwrap_or(0) as f64;
            let number_cpus = stats.cpu_stats.online_cpus.unwrap_or(1) as f64;

            if system_delta > 0.0 && cpu_delta > 0.0 {
                Some((cpu_delta / system_delta) * number_cpus * 100.0)
            } else {
                None
            }
        };

        // Memory stats
        let (memory_usage, memory_limit, memory_percent) = {
            let usage_mb = stats.memory_stats.usage.unwrap_or(0) as f64 / 1024.0 / 1024.0;
            let limit_mb = stats.memory_stats.limit.unwrap_or(0) as f64 / 1024.0 / 1024.0;
            let percent = if limit_mb > 0.0 {
                Some((usage_mb / limit_mb) * 100.0)
            } else {
                None
            };
            (Some(usage_mb), Some(limit_mb), percent)
        };

        // Network stats
        let (network_rx, network_tx) = if let Some(networks) = &stats.networks {
            let rx_bytes: u64 = networks.values().map(|n| n.rx_bytes).sum();
            let tx_bytes: u64 = networks.values().map(|n| n.tx_bytes).sum();
            (
                Some(rx_bytes as f64 / 1024.0 / 1024.0),
                Some(tx_bytes as f64 / 1024.0 / 1024.0),
            )
        } else {
            (None, None)
        };

        // Block I/O stats
        let (block_read, block_write) = {
            let read_bytes: u64 = stats
                .blkio_stats
                .io_service_bytes_recursive
                .as_ref()
                .map(|stats| {
                    stats
                        .iter()
                        .filter(|s| s.op == "read" || s.op == "Read")
                        .map(|s| s.value)
                        .sum()
                })
                .unwrap_or(0);

            let write_bytes: u64 = stats
                .blkio_stats
                .io_service_bytes_recursive
                .as_ref()
                .map(|stats| {
                    stats
                        .iter()
                        .filter(|s| s.op == "write" || s.op == "Write")
                        .map(|s| s.value)
                        .sum()
                })
                .unwrap_or(0);

            (
                Some(read_bytes as f64 / 1024.0 / 1024.0),
                Some(write_bytes as f64 / 1024.0 / 1024.0),
            )
        };

        Some(ResourceStats {
            cpu_usage_percent: cpu_usage,
            memory_usage_mb: memory_usage,
            memory_limit_mb: memory_limit,
            memory_percent,
            network_rx_mb: network_rx,
            network_tx_mb: network_tx,
            block_read_mb: block_read,
            block_write_mb: block_write,
        })
    } else {
        None
    }
}

/// Check HTTP service health
async fn check_http_service(name: &str, url: &str) -> ServiceStatus {
    let client = reqwest::Client::new();

    match client
        .head(url)
        .timeout(std::time::Duration::from_secs(2))
        .send()
        .await
    {
        Ok(response) => {
            let status = if response.status().is_success()
                || response.status().as_u16() == 405
                || response.status().is_redirection()
            {
                "operational"
            } else {
                "down"
            };

            ServiceStatus {
                name: name.to_string(),
                status: status.to_string(),
                container_state: None,
                health: Some(format!("HTTP {}", response.status().as_u16())),
                uptime: None,
                resources: None, // HTTP services don't have Docker stats
            }
        }
        Err(_) => ServiceStatus {
            name: name.to_string(),
            status: "down".to_string(),
            container_state: None,
            health: None,
            uptime: None,
            resources: None,
        },
    }
}

/// Health check endpoint
async fn health() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "monitoring-service",
        "version": env!("CARGO_PKG_VERSION")
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    // Connect to Docker
    let docker = Docker::connect_with_local_defaults().expect("Failed to connect to Docker");

    info!("Connected to Docker daemon");

    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "8090".to_string())
        .parse::<u16>()
        .expect("PORT must be a valid number");

    info!("Starting monitoring-service on port {}", port);

    HttpServer::new(move || {
        let cors = Cors::permissive(); // Allow all origins for development

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(docker.clone()))
            .route("/health", web::get().to(health))
            .route("/api/v1/services", web::get().to(get_services_status))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
