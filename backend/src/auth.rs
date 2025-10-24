use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Serialize, Deserialize};
use chrono::{Utc, Duration};
use crate::error::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // 用户ID
    pub exp: usize,  // 过期时间
    pub iat: usize,  // 签发时间
}

pub struct JwtService {
    secret: String,
}

impl JwtService {
    pub fn new(secret: String) -> Self {
        Self { secret }
    }
    
    /// 生成JWT token
    pub fn create_token(&self, user_id: &str, expires_in_sec: i64) -> Result<String, AppError> {
        let expiration = Utc::now() + Duration::seconds(expires_in_sec);
        let claims = Claims {
            sub: user_id.to_string(),
            exp: expiration.timestamp() as usize,
            iat: Utc::now().timestamp() as usize,
        };
        
        encode(
            &Header::default(), 
            &claims, 
            &EncodingKey::from_secret(self.secret.as_ref())
        ).map_err(|e| AppError::JwtError(e.to_string()))
    }
    
    /// 验证JWT token
    pub fn verify_token(&self, token: &str) -> Result<String, AppError> {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.secret.as_ref()),
            &Validation::default()
        ).map_err(|e| AppError::JwtError(e.to_string()))?;
        
        Ok(token_data.claims.sub)
    }
}