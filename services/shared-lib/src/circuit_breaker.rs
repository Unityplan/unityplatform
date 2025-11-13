//! Circuit breaker pattern for resilient inter-service communication
//!
//! Prevents cascading failures when calling external services or other pods.
//! Implements the three-state circuit breaker pattern:
//! - **Closed**: Normal operation, requests pass through
//! - **Open**: Service degraded, requests fail fast without calling downstream
//! - **HalfOpen**: Testing if service recovered, limited requests allowed
//!
//! # Example
//!
//! ```rust
//! use shared_lib::CircuitBreaker;
//! use std::time::Duration;
//!
//! #[tokio::main]
//! async fn main() {
//!     let breaker = CircuitBreaker::new(5, Duration::from_secs(30));
//!     
//!     match breaker.call(async {
//!         // Call external service
//!         reqwest::get("http://other-pod.example.com/api/users")
//!             .await?
//!             .json::<Vec<User>>()
//!             .await
//!     }).await {
//!         Ok(users) => println!("Got {} users", users.len()),
//!         Err(e) => println!("Circuit breaker: {}", e),
//!     }
//! }
//! ```

use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

/// Circuit breaker state
#[derive(Debug, Clone, PartialEq)]
pub enum CircuitState {
    /// Circuit is closed - requests pass through normally
    Closed,
    /// Circuit is open - requests fail fast without calling downstream
    Open,
    /// Circuit is half-open - testing if service recovered
    HalfOpen,
}

/// Circuit breaker error type
#[derive(Debug, thiserror::Error)]
pub enum CircuitBreakerError<E> {
    /// Circuit is open, request rejected
    #[error("Circuit breaker is open - service unavailable")]
    Open,
    
    /// Underlying call failed
    #[error("Call failed: {0}")]
    CallFailed(E),
}

/// Circuit breaker for resilient service calls
///
/// Tracks failure rate and opens circuit when threshold exceeded.
/// After timeout period, transitions to half-open to test recovery.
///
/// # Configuration
///
/// - `failure_threshold`: Number of consecutive failures before opening (default: 5)
/// - `timeout`: Duration to wait before attempting recovery (default: 30s)
///
/// # States
///
/// - **Closed**: Normal operation
///   - Success: Reset failure count
///   - Failure: Increment failure count
///   - Failure count >= threshold: Open circuit
///
/// - **Open**: Fail-fast mode
///   - All requests rejected immediately
///   - After timeout: Transition to half-open
///
/// - **HalfOpen**: Recovery test
///   - Allow limited requests through
///   - Success: Close circuit
///   - Failure: Re-open circuit
pub struct CircuitBreaker {
    failure_count: Arc<RwLock<u32>>,
    failure_threshold: u32,
    timeout: Duration,
    last_failure_time: Arc<RwLock<Option<Instant>>>,
    state: Arc<RwLock<CircuitState>>,
}

impl CircuitBreaker {
    /// Create a new circuit breaker
    ///
    /// # Arguments
    ///
    /// * `failure_threshold` - Number of consecutive failures before opening circuit
    /// * `timeout` - Duration to wait before testing recovery (half-open state)
    ///
    /// # Example
    ///
    /// ```rust
    /// use shared_lib::CircuitBreaker;
    /// use std::time::Duration;
    ///
    /// // Open after 5 failures, test recovery after 30 seconds
    /// let breaker = CircuitBreaker::new(5, Duration::from_secs(30));
    /// ```
    pub fn new(failure_threshold: u32, timeout: Duration) -> Self {
        Self {
            failure_count: Arc::new(RwLock::new(0)),
            failure_threshold,
            timeout,
            last_failure_time: Arc::new(RwLock::new(None)),
            state: Arc::new(RwLock::new(CircuitState::Closed)),
        }
    }

    /// Create a circuit breaker with development defaults
    ///
    /// - Threshold: 3 failures (lenient for local development)
    /// - Timeout: 10 seconds (fast recovery for testing)
    pub fn development() -> Self {
        Self::new(3, Duration::from_secs(10))
    }

    /// Create a circuit breaker with production defaults
    ///
    /// - Threshold: 5 failures (stricter for production)
    /// - Timeout: 30 seconds (cautious recovery)
    pub fn production() -> Self {
        Self::new(5, Duration::from_secs(30))
    }

    /// Execute a call through the circuit breaker
    ///
    /// # Returns
    ///
    /// - `Ok(T)`: Call succeeded
    /// - `Err(CircuitBreakerError::Open)`: Circuit is open, call rejected
    /// - `Err(CircuitBreakerError::CallFailed(E))`: Call failed with error E
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use shared_lib::CircuitBreaker;
    /// use std::time::Duration;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let breaker = CircuitBreaker::new(5, Duration::from_secs(30));
    ///     
    ///     match breaker.call(async {
    ///         // Call potentially failing service
    ///         Ok::<_, String>("result".to_string())
    ///     }).await {
    ///         Ok(result) => println!("Success: {}", result),
    ///         Err(e) => println!("Error: {}", e),
    ///     }
    /// }
    /// ```
    pub async fn call<F, T, E>(&self, future: F) -> Result<T, CircuitBreakerError<E>>
    where
        F: std::future::Future<Output = Result<T, E>>,
    {
        // Check if we should transition from Open to HalfOpen
        self.check_timeout().await;

        // Check current state
        let state = self.state.read().await.clone();
        match state {
            CircuitState::Open => {
                // Circuit is open - reject request immediately
                Err(CircuitBreakerError::Open)
            }
            CircuitState::Closed | CircuitState::HalfOpen => {
                // Attempt the call
                match future.await {
                    Ok(result) => {
                        // Success - reset failure count and close circuit
                        self.on_success().await;
                        Ok(result)
                    }
                    Err(error) => {
                        // Failure - increment count and possibly open circuit
                        self.on_failure().await;
                        Err(CircuitBreakerError::CallFailed(error))
                    }
                }
            }
        }
    }

    /// Get current circuit state
    pub async fn state(&self) -> CircuitState {
        self.state.read().await.clone()
    }

    /// Get current failure count
    pub async fn failure_count(&self) -> u32 {
        *self.failure_count.read().await
    }

    /// Check if timeout has elapsed and transition to HalfOpen if needed
    async fn check_timeout(&self) {
        let last_failure = self.last_failure_time.read().await;
        
        if let Some(last_time) = *last_failure {
            if last_time.elapsed() >= self.timeout {
                // Timeout elapsed - transition to HalfOpen
                let mut state = self.state.write().await;
                if *state == CircuitState::Open {
                    *state = CircuitState::HalfOpen;
                    tracing::info!(
                        "Circuit breaker transitioning to HalfOpen (testing recovery after {}s)",
                        self.timeout.as_secs()
                    );
                }
            }
        }
    }

    /// Handle successful call
    async fn on_success(&self) {
        // Reset failure count
        let mut count = self.failure_count.write().await;
        *count = 0;

        // Close circuit if it was half-open
        let mut state = self.state.write().await;
        if *state == CircuitState::HalfOpen {
            *state = CircuitState::Closed;
            tracing::info!("Circuit breaker closed (service recovered)");
        }
    }

    /// Handle failed call
    async fn on_failure(&self) {
        // Increment failure count
        let mut count = self.failure_count.write().await;
        *count += 1;
        let current_count = *count;

        // Record failure time
        let mut last_failure = self.last_failure_time.write().await;
        *last_failure = Some(Instant::now());

        // Check if we should open the circuit
        if current_count >= self.failure_threshold {
            let mut state = self.state.write().await;
            *state = CircuitState::Open;
            tracing::warn!(
                "Circuit breaker opened ({} consecutive failures, timeout: {}s)",
                current_count,
                self.timeout.as_secs()
            );
        }
    }

    /// Manually reset the circuit breaker (for testing/admin purposes)
    pub async fn reset(&self) {
        let mut count = self.failure_count.write().await;
        *count = 0;
        
        let mut state = self.state.write().await;
        *state = CircuitState::Closed;
        
        let mut last_failure = self.last_failure_time.write().await;
        *last_failure = None;
        
        tracing::info!("Circuit breaker manually reset");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_circuit_breaker_closed_on_success() {
        let breaker = CircuitBreaker::new(3, Duration::from_secs(1));
        
        let result = breaker.call(async { Ok::<_, String>("success") }).await;
        
        assert!(result.is_ok());
        assert_eq!(breaker.state().await, CircuitState::Closed);
        assert_eq!(breaker.failure_count().await, 0);
    }

    #[tokio::test]
    async fn test_circuit_breaker_opens_after_threshold() {
        let breaker = CircuitBreaker::new(3, Duration::from_secs(1));
        
        // First 3 failures should open the circuit
        for _ in 0..3 {
            let _ = breaker.call(async { Err::<String, _>("error") }).await;
        }
        
        assert_eq!(breaker.state().await, CircuitState::Open);
        assert_eq!(breaker.failure_count().await, 3);
    }

    #[tokio::test]
    async fn test_circuit_breaker_rejects_when_open() {
        let breaker = CircuitBreaker::new(3, Duration::from_secs(1));
        
        // Open the circuit
        for _ in 0..3 {
            let _ = breaker.call(async { Err::<String, _>("error") }).await;
        }
        
        // Next call should be rejected
        let result = breaker.call(async { Ok::<_, String>("success") }).await;
        
        assert!(matches!(result, Err(CircuitBreakerError::Open)));
    }

    #[tokio::test]
    async fn test_circuit_breaker_transitions_to_half_open() {
        let breaker = CircuitBreaker::new(3, Duration::from_millis(100));
        
        // Open the circuit
        for _ in 0..3 {
            let _ = breaker.call(async { Err::<String, _>("error") }).await;
        }
        
        assert_eq!(breaker.state().await, CircuitState::Open);
        
        // Wait for timeout
        tokio::time::sleep(Duration::from_millis(150)).await;
        
        // Should transition to HalfOpen on next call
        breaker.check_timeout().await;
        assert_eq!(breaker.state().await, CircuitState::HalfOpen);
    }

    #[tokio::test]
    async fn test_circuit_breaker_closes_on_recovery() {
        let breaker = CircuitBreaker::new(3, Duration::from_millis(100));
        
        // Open the circuit
        for _ in 0..3 {
            let _ = breaker.call(async { Err::<String, _>("error") }).await;
        }
        
        // Wait for timeout
        tokio::time::sleep(Duration::from_millis(150)).await;
        
        // Successful call in HalfOpen should close circuit
        let result = breaker.call(async { Ok::<_, String>("success") }).await;
        
        assert!(result.is_ok());
        assert_eq!(breaker.state().await, CircuitState::Closed);
        assert_eq!(breaker.failure_count().await, 0);
    }

    #[tokio::test]
    async fn test_circuit_breaker_reset() {
        let breaker = CircuitBreaker::new(3, Duration::from_secs(1));
        
        // Open the circuit
        for _ in 0..3 {
            let _ = breaker.call(async { Err::<String, _>("error") }).await;
        }
        
        assert_eq!(breaker.state().await, CircuitState::Open);
        
        // Reset
        breaker.reset().await;
        
        assert_eq!(breaker.state().await, CircuitState::Closed);
        assert_eq!(breaker.failure_count().await, 0);
    }

    #[tokio::test]
    async fn test_development_defaults() {
        let breaker = CircuitBreaker::development();
        
        // Should open after 3 failures
        for _ in 0..3 {
            let _ = breaker.call(async { Err::<String, _>("error") }).await;
        }
        
        assert_eq!(breaker.state().await, CircuitState::Open);
    }

    #[tokio::test]
    async fn test_production_defaults() {
        let breaker = CircuitBreaker::production();
        
        // Should stay closed after 3 failures
        for _ in 0..3 {
            let _ = breaker.call(async { Err::<String, _>("error") }).await;
        }
        
        assert_eq!(breaker.state().await, CircuitState::Closed);
        
        // Should open after 5 failures
        for _ in 0..2 {
            let _ = breaker.call(async { Err::<String, _>("error") }).await;
        }
        
        assert_eq!(breaker.state().await, CircuitState::Open);
    }
}
