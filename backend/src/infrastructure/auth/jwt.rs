use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

const ACCESS_TOKEN_EXPIRATION_MINUTES: i64 = 15;
const REFRESH_TOKEN_EXPIRATION_DAYS: i64 = 30;
const SSE_TOKEN_EXPIRATION_SECONDS: i64 = 60; // SSE専用トークン: 1分

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // user_id
    pub exp: i64,    // expiration time
    pub iat: i64,    // issued at
    pub token_type: TokenType,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TokenType {
    Access,
    Refresh,
    Sse, // SSE接続専用の短命トークン
}

pub struct JwtService {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
}

impl JwtService {
    pub fn new(secret: &str) -> Self {
        Self {
            encoding_key: EncodingKey::from_secret(secret.as_bytes()),
            decoding_key: DecodingKey::from_secret(secret.as_bytes()),
        }
    }

    pub fn generate_access_token(&self, user_id: Uuid) -> Result<String, jsonwebtoken::errors::Error> {
        let now = Utc::now();
        let exp = now + Duration::minutes(ACCESS_TOKEN_EXPIRATION_MINUTES);

        let claims = Claims {
            sub: user_id.to_string(),
            exp: exp.timestamp(),
            iat: now.timestamp(),
            token_type: TokenType::Access,
        };

        encode(&Header::default(), &claims, &self.encoding_key)
    }

    pub fn generate_refresh_token(&self, user_id: Uuid) -> Result<String, jsonwebtoken::errors::Error> {
        let now = Utc::now();
        let exp = now + Duration::days(REFRESH_TOKEN_EXPIRATION_DAYS);

        let claims = Claims {
            sub: user_id.to_string(),
            exp: exp.timestamp(),
            iat: now.timestamp(),
            token_type: TokenType::Refresh,
        };

        encode(&Header::default(), &claims, &self.encoding_key)
    }

    pub fn verify_token(&self, token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
        let token_data = decode::<Claims>(token, &self.decoding_key, &Validation::default())?;
        Ok(token_data.claims)
    }

    pub fn verify_access_token(&self, token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
        let claims = self.verify_token(token)?;
        if claims.token_type != TokenType::Access {
            return Err(jsonwebtoken::errors::Error::from(
                jsonwebtoken::errors::ErrorKind::InvalidToken,
            ));
        }
        Ok(claims)
    }

    pub fn verify_refresh_token(&self, token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
        let claims = self.verify_token(token)?;
        if claims.token_type != TokenType::Refresh {
            return Err(jsonwebtoken::errors::Error::from(
                jsonwebtoken::errors::ErrorKind::InvalidToken,
            ));
        }
        Ok(claims)
    }

    pub fn generate_sse_token(&self, user_id: Uuid) -> Result<String, jsonwebtoken::errors::Error> {
        let now = Utc::now();
        let exp = now + Duration::seconds(SSE_TOKEN_EXPIRATION_SECONDS);

        let claims = Claims {
            sub: user_id.to_string(),
            exp: exp.timestamp(),
            iat: now.timestamp(),
            token_type: TokenType::Sse,
        };

        encode(&Header::default(), &claims, &self.encoding_key)
    }

    pub fn verify_sse_token(&self, token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
        let claims = self.verify_token(token)?;
        if claims.token_type != TokenType::Sse {
            return Err(jsonwebtoken::errors::Error::from(
                jsonwebtoken::errors::ErrorKind::InvalidToken,
            ));
        }
        Ok(claims)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_and_verify_access_token() {
        let jwt_service = JwtService::new("test_secret");
        let user_id = Uuid::new_v4();

        let token = jwt_service.generate_access_token(user_id).unwrap();
        let claims = jwt_service.verify_access_token(&token).unwrap();

        assert_eq!(claims.sub, user_id.to_string());
        assert_eq!(claims.token_type, TokenType::Access);
    }

    #[test]
    fn test_generate_and_verify_refresh_token() {
        let jwt_service = JwtService::new("test_secret");
        let user_id = Uuid::new_v4();

        let token = jwt_service.generate_refresh_token(user_id).unwrap();
        let claims = jwt_service.verify_refresh_token(&token).unwrap();

        assert_eq!(claims.sub, user_id.to_string());
        assert_eq!(claims.token_type, TokenType::Refresh);
    }

    #[test]
    fn test_verify_wrong_token_type() {
        let jwt_service = JwtService::new("test_secret");
        let user_id = Uuid::new_v4();

        let access_token = jwt_service.generate_access_token(user_id).unwrap();
        let result = jwt_service.verify_refresh_token(&access_token);

        assert!(result.is_err());
    }
}
