use redis::{AsyncCommands, Client};
use futures_util::stream::StreamExt;
use tokio::sync::broadcast;
use crate::error::AppError;

#[derive(Clone)]
pub struct RedisPubSub {
    pub tx: broadcast::Sender<String>,
    pub client: Client,
}

impl RedisPubSub {
    pub async fn new(redis_url: &str) -> Result<Self, AppError> {
        let client = Client::open(redis_url)
            .map_err(|e| AppError::RedisError(e))?;
            
        let (tx, _) = broadcast::channel(100);
        
        Ok(Self { tx, client })
    }
    
    pub async fn subscribe(&self, channel: &str) -> Result<(), AppError> {
        let mut connection = self.client.get_async_connection().await
            .map_err(|e| AppError::RedisError(e))?;
        let mut pubsub = connection.into_pubsub();
        pubsub.subscribe(channel)
            .await
            .map_err(|e| AppError::RedisError(e))?;
            
        let tx = self.tx.clone();
        
        tokio::spawn(async move {
            let mut stream = pubsub.on_message();
            while let Some(msg) = stream.next().await {
                if let Ok(payload) = msg.get_payload::<String>() {
                    let _ = tx.send(payload);
                }
            }
        });
        
        Ok(())
    }
    
    pub async fn publish(&mut self, channel: &str, message: &str) -> Result<(), AppError> {
        let mut connection = self.client.get_async_connection().await
            .map_err(|e| AppError::RedisError(e))?;
        connection.publish(channel, message)
            .await
            .map_err(|e| AppError::RedisError(e))?;
            
        Ok(())
    }
    
    pub async fn set(&mut self, key: &str, value: &str, ttl_seconds: usize) -> Result<(), AppError> {
        let mut connection = self.client.get_async_connection().await
            .map_err(|e| AppError::RedisError(e))?;
        let _: () = connection.set_ex(key, value, ttl_seconds)
            .await
            .map_err(|e| AppError::RedisError(e))?;
            
        Ok(())
    }
    
    pub async fn get(&mut self, key: &str) -> Result<Option<String>, AppError> {
        let mut connection = self.client.get_async_connection().await
            .map_err(|e| AppError::RedisError(e))?;
        let result: Option<String> = connection.get(key)
            .await
            .map_err(|e| AppError::RedisError(e))?;
            
        Ok(result)
    }
}