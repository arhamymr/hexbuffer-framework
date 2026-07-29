use async_trait::async_trait;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use crate::domain::auth::Claims;
use crate::domain::user::{DomainError, User};
use crate::ports::outbound::token_service::TokenService;

pub struct JwtTokenService {
    secret: String,
    expiration_secs: i64,
}

impl JwtTokenService {
    pub fn new(secret: String, expiration_secs: i64) -> Self {
        Self { secret, expiration_secs }
    }
}

#[async_trait]
impl TokenService for JwtTokenService {
    async fn generate_token(&self, user: &User) -> Result<String, DomainError> {
        let now = chrono::Utc::now().timestamp();
        let claims = Claims {
            sub: user.id.to_string(),
            email: user.email.to_string(),
            name: user.name.clone(),
            exp: now + self.expiration_secs,
            iat: now,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )
        .map_err(|e| DomainError::Unauthorized(format!("Failed to generate JWT: {}", e)))
    }

    async fn verify_token(&self, token: &str) -> Result<Claims, DomainError> {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|e| DomainError::InvalidToken(format!("JWT validation failed: {}", e)))?;

        Ok(token_data.claims)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_jwt_generate_and_verify() {
        let service = JwtTokenService::new("super_secret_key_123".to_string(), 3600);
        let user = User {
            id: UserId::new("usr_100"),
            name: "Alice".to_string(),
            email: Email::new("alice@example.com").unwrap(),
            password_hash: None,
        };

        let token = service.generate_token(&user).await.unwrap();
        assert!(!token.is_empty());

        let claims = service.verify_token(&token).await.unwrap();
        assert_eq!(claims.sub, user.id.to_string());
        assert_eq!(claims.email, user.email.to_string());
    }
}
