use anyhow::Result;
use jsonwebtoken::{
    decode, encode, get_current_timestamp, DecodingKey, EncodingKey, Header, Validation,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub exp: usize,
    pub iat: usize,
    pub sub: String,
}

impl Claims {
    pub fn new(sub: &str, lifetime_secs: usize) -> Self {
        let now = get_current_timestamp() as usize;
        Self {
            sub: sub.to_string(),
            iat: get_current_timestamp() as usize,
            exp: now + lifetime_secs,
        }
    }
}

pub struct TokenHandler {
    signing_key_enc: EncodingKey,
    signing_key_dec: DecodingKey,
}

impl TokenHandler {
    pub fn new(signing_key: &[u8]) -> Self {
        Self {
            signing_key_enc: EncodingKey::from_secret(signing_key),
            signing_key_dec: DecodingKey::from_secret(signing_key),
        }
    }

    pub fn encode<T>(&self, claims: &T) -> Result<String>
    where
        T: Serialize,
    {
        let token = encode(&Header::default(), claims, &self.signing_key_enc)?;
        Ok(token)
    }

    pub fn decode<T>(&self, token: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let claims = decode(token, &self.signing_key_dec, &Validation::default())?;
        Ok(claims.claims)
    }
}
