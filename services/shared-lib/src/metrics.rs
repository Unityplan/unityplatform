//! Prometheus metrics collection and formatting
//!
//! This module provides standardized metrics collection for all services.
//! It handles HTTP request metrics, error tracking, and custom service metrics.

use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

/// Metrics collector for a service
#[derive(Clone, Debug)]
pub struct MetricsCollector {
    service_name: String,
    version: String,
    http_requests: Arc<RwLock<HashMap<String, AtomicU64>>>,
    http_durations: Arc<RwLock<HashMap<String, Vec<f64>>>>,
    errors: Arc<RwLock<HashMap<String, AtomicU64>>>,
}

impl MetricsCollector {
    /// Create a new metrics collector
    pub fn new(service_name: impl Into<String>, version: impl Into<String>) -> Self {
        Self {
            service_name: service_name.into(),
            version: version.into(),
            http_requests: Arc::new(RwLock::new(HashMap::new())),
            http_durations: Arc::new(RwLock::new(HashMap::new())),
            errors: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Record an HTTP request
    pub fn record_request(&self, method: &str, path: &str, status: u16, duration_seconds: f64) {
        // Record request count
        let key = format!(
            "method=\"{}\",path=\"{}\",status=\"{}\"",
            method, path, status
        );
        let requests = self.http_requests.read();
        if let Some(counter) = requests.get(&key) {
            counter.fetch_add(1, Ordering::Relaxed);
        } else {
            drop(requests);
            let mut requests = self.http_requests.write();
            requests.insert(key.clone(), AtomicU64::new(1));
        }

        // Record duration
        let duration_key = format!("method=\"{}\",path=\"{}\"", method, path);
        let mut durations = self.http_durations.write();
        durations
            .entry(duration_key)
            .or_insert_with(Vec::new)
            .push(duration_seconds);
    }

    /// Record an error
    pub fn record_error(&self, error_type: &str) {
        let key = format!("type=\"{}\"", error_type);
        let errors = self.errors.read();
        if let Some(counter) = errors.get(&key) {
            counter.fetch_add(1, Ordering::Relaxed);
        } else {
            drop(errors);
            let mut errors = self.errors.write();
            errors.insert(key.clone(), AtomicU64::new(1));
        }
    }

    /// Generate Prometheus-format metrics text
    pub fn generate_prometheus_metrics(
        &self,
        db_pool_size: Option<u32>,
        db_pool_idle: Option<usize>,
    ) -> String {
        let mut output = String::new();

        // Service info
        output.push_str(&format!(
            "# HELP {}_info Service information\n\
             # TYPE {}_info gauge\n\
             {}_info{{version=\"{}\"}} 1\n\n",
            self.service_name, self.service_name, self.service_name, self.version
        ));

        // Database pool metrics (if provided)
        if let (Some(size), Some(idle)) = (db_pool_size, db_pool_idle) {
            let active = size.saturating_sub(idle as u32);
            output.push_str(&format!(
                "# HELP {}_db_pool_size Database connection pool size\n\
                 # TYPE {}_db_pool_size gauge\n\
                 {}_db_pool_size {}\n\n\
                 # HELP {}_db_pool_idle Database connection pool idle connections\n\
                 # TYPE {}_db_pool_idle gauge\n\
                 {}_db_pool_idle {}\n\n\
                 # HELP {}_db_pool_active Database connection pool active connections\n\
                 # TYPE {}_db_pool_active gauge\n\
                 {}_db_pool_active {}\n\n",
                self.service_name,
                self.service_name,
                self.service_name,
                size,
                self.service_name,
                self.service_name,
                self.service_name,
                idle,
                self.service_name,
                self.service_name,
                self.service_name,
                active
            ));
        }

        // HTTP request metrics
        output.push_str(&format!(
            "# HELP {}_http_requests_total Total HTTP requests\n\
             # TYPE {}_http_requests_total counter\n",
            self.service_name, self.service_name
        ));
        let requests = self.http_requests.read();
        for (labels, counter) in requests.iter() {
            let count = counter.load(Ordering::Relaxed);
            output.push_str(&format!(
                "{}_http_requests_total{{{}}} {}\n",
                self.service_name, labels, count
            ));
        }
        output.push('\n');

        // HTTP request duration metrics (simple histogram)
        output.push_str(&format!(
            "# HELP {}_http_request_duration_seconds HTTP request duration in seconds\n\
             # TYPE {}_http_request_duration_seconds histogram\n",
            self.service_name, self.service_name
        ));
        let durations = self.http_durations.read();
        for (labels, duration_list) in durations.iter() {
            if duration_list.is_empty() {
                continue;
            }

            // Calculate histogram buckets
            let mut buckets = vec![0.001, 0.005, 0.01, 0.05, 0.1, 0.5, 1.0, 5.0, 10.0];
            buckets.sort_by(|a, b| a.partial_cmp(b).unwrap());

            let mut counts = vec![0u64; buckets.len()];
            let mut sum = 0.0;
            let count = duration_list.len() as u64;

            for &duration in duration_list.iter() {
                sum += duration;
                for (i, &bucket) in buckets.iter().enumerate() {
                    if duration <= bucket {
                        counts[i] += 1;
                    }
                }
            }

            // Output histogram buckets
            for (i, &bucket) in buckets.iter().enumerate() {
                output.push_str(&format!(
                    "{}_http_request_duration_seconds_bucket{{{},le=\"{}\"}} {}\n",
                    self.service_name, labels, bucket, counts[i]
                ));
            }
            output.push_str(&format!(
                "{}_http_request_duration_seconds_bucket{{{},le=\"+Inf\"}} {}\n",
                self.service_name, labels, count
            ));
            output.push_str(&format!(
                "{}_http_request_duration_seconds_sum{{{}}} {}\n",
                self.service_name, labels, sum
            ));
            output.push_str(&format!(
                "{}_http_request_duration_seconds_count{{{}}} {}\n",
                self.service_name, labels, count
            ));
        }
        output.push('\n');

        // Error metrics
        output.push_str(&format!(
            "# HELP {}_errors_total Total errors by type\n\
             # TYPE {}_errors_total counter\n",
            self.service_name, self.service_name
        ));
        let errors = self.errors.read();
        for (labels, counter) in errors.iter() {
            let count = counter.load(Ordering::Relaxed);
            output.push_str(&format!(
                "{}_errors_total{{{}}} {}\n",
                self.service_name, labels, count
            ));
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metrics_collector() {
        let collector = MetricsCollector::new("test_service", "1.0.0");

        // Record some requests
        collector.record_request("GET", "/api/v1/health", 200, 0.001);
        collector.record_request("GET", "/api/v1/health", 200, 0.002);
        collector.record_request("POST", "/api/v1/users", 201, 0.050);
        collector.record_request("GET", "/api/v1/users", 500, 0.100);

        // Record some errors
        collector.record_error("DatabaseError");
        collector.record_error("ValidationError");
        collector.record_error("DatabaseError");

        // Generate metrics
        let metrics = collector.generate_prometheus_metrics(Some(20), Some(15));

        // Verify metrics content
        assert!(metrics.contains("test_service_info{version=\"1.0.0\"} 1"));
        assert!(metrics.contains("test_service_db_pool_size 20"));
        assert!(metrics.contains("test_service_db_pool_idle 15"));
        assert!(metrics.contains("test_service_db_pool_active 5"));
        assert!(metrics.contains("test_service_http_requests_total"));
        assert!(metrics.contains("test_service_http_request_duration_seconds"));
        assert!(metrics.contains("test_service_errors_total"));
    }
}
