use async_nats::Client;
use crate::error::{AppError, Result};

#[derive(Clone)]
pub struct NatsClient {
    client: Client,
    cluster_name: String,
}

impl NatsClient {
    pub async fn new(nats_url: &str, cluster_name: String) -> Result<Self> {
        let client = async_nats::connect(nats_url)
            .await
            .map_err(|e| AppError::Nats(e.to_string()))?;

        tracing::info!("Connected to NATS at {}", nats_url);

        Ok(Self {
            client,
            cluster_name,
        })
    }

    pub fn client(&self) -> &Client {
        &self.client
    }

    pub fn cluster_name(&self) -> &str {
        &self.cluster_name
    }

    pub async fn publish(&self, subject: &str, payload: impl Into<bytes::Bytes>) -> Result<()> {
        self.client
            .publish(subject.to_string(), payload.into())
            .await
            .map_err(|e| AppError::Nats(e.to_string()))?;
        Ok(())
    }

    pub async fn request(
        &self,
        subject: &str,
        payload: impl Into<bytes::Bytes>,
    ) -> Result<async_nats::Message> {
        self.client
            .request(subject.to_string(), payload.into())
            .await
            .map_err(|e| AppError::Nats(e.to_string()))
    }

    pub async fn subscribe(&self, subject: &str) -> Result<async_nats::Subscriber> {
        self.client
            .subscribe(subject.to_string())
            .await
            .map_err(|e| AppError::Nats(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore] // Requires NATS to be running
    async fn test_nats_connection() {
        let client = NatsClient::new(
            "nats://localhost:4222",
            "unityplan-global".to_string()
        ).await.unwrap();

        assert!(client.publish("test.subject", b"test message").await.is_ok());
    }
}
