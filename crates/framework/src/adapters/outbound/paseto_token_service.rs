use async_trait::async_trait;
use pasetors::claims::{Claims as PasetoClaims, ClaimsValidationRules};
use pasetors::keys::SymmetricKey;
use pasetors::local;
use pasetors::token::UntrustedToken;
use pasetors::version4::V4;
use pasetors::Local;

use crate::domain::auth::Claims;
use crate::domain::user::{DomainError, User};
use crate::ports::outbound::token_service::TokenService;

pub struct PasetoTokenService {
    key: SymmetricKey<V4>,
    expiration_secs: i64,
}

impl PasetoTokenService {
    pub fn new(secret_32_bytes: &[u8; 32], expiration_secs: i64) -> Result<Self, DomainError> {
        let key = SymmetricKey::<V4>::from(secret_32_bytes)
            .map_err(|e| DomainError::Unauthorized(format!("Invalid PASETO key: {}", e)))?;
        Ok(Self { key, expiration_secs })
    }
}

#[async_trait]
impl TokenService for PasetoTokenService {
    async fn generate_token(&self, user: &User) -> Result<String, DomainError> {
        let mut paseto_claims = PasetoClaims::new()
            .map_err(|e| DomainError::Unauthorized(format!("PASETO claims creation failed: {}", e)))?;

        paseto_claims.subject(&user.id)
            .map_err(|e| DomainError::Unauthorized(format!("PASETO claim set sub failed: {}", e)))?;

        paseto_claims.add_additional("email", user.email.as_str())
            .map_err(|e| DomainError::Unauthorized(format!("PASETO claim set email failed: {}", e)))?;

        paseto_claims.add_additional("name", user.name.as_str())
            .map_err(|e| DomainError::Unauthorized(format!("PASETO claim set name failed: {}", e)))?;

        let now = chrono::Utc::now();
        let exp = now + chrono::Duration::seconds(self.expiration_secs);
        paseto_claims.expiration(&exp.to_rfc3339())
            .map_err(|e| DomainError::Unauthorized(format!("PASETO claim set exp failed: {}", e)))?;

        local::encrypt(&self.key, &paseto_claims, None, None)
            .map_err(|e| DomainError::Unauthorized(format!("Failed to encrypt PASETO token: {}", e)))
    }

    async fn verify_token(&self, token: &str) -> Result<Claims, DomainError> {
        let untrusted = UntrustedToken::<Local, V4>::try_from(token)
            .map_err(|e| DomainError::InvalidToken(format!("Invalid PASETO token format: {}", e)))?;

        let validation = ClaimsValidationRules::new();
        let trusted = local::decrypt(&self.key, &untrusted, &validation, None, None)
            .map_err(|e| DomainError::InvalidToken(format!("PASETO token validation failed: {}", e)))?;

        let payload_json = trusted.payload();
        let val: serde_json::Value = serde_json::from_str(payload_json)
            .map_err(|e| DomainError::InvalidToken(format!("Failed to parse PASETO payload: {}", e)))?;

        let sub = val.get("sub").and_then(|v| v.as_str()).unwrap_or("").to_string();
        let email = val.get("email").and_then(|v| v.as_str()).unwrap_or("").to_string();
        let name = val.get("name").and_then(|v| v.as_str()).unwrap_or("").to_string();

        let now = chrono::Utc::now().timestamp();
        Ok(Claims {
            sub,
            email,
            name,
            exp: now + self.expiration_secs,
            iat: now,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_paseto_generate_and_verify() {
        let secret = b"YELLOW SUBMARINE, BLACK SUBMARIN";
        let service = PasetoTokenService::new(secret, 3600).unwrap();

        let user = User {
            id: "usr_200".to_string(),
            name: "Bob".to_string(),
            email: "bob@example.com".to_string(),
        };

        let token = service.generate_token(&user).await.unwrap();
        assert!(token.starts_with("v4.local."));

        let claims = service.verify_token(&token).await.unwrap();
        assert_eq!(claims.sub, user.id);
        assert_eq!(claims.email, user.email);
    }
}
