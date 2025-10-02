use serde::{Serialize, Deserialize};
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};

/// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,    // usename
    pub tutur_id: i32,
    pub exp: usize,
}


const JWT_SECRET: &str = "ezytutors_secret";

pub fn generate_jwt(username: &str, tutor_id: i32) -> Result<String, jsonwebtoken::errors::Error> {
    let now = chrono::Utc::now();
    let exp = now + chrono::Duration::weeks(1);
    let claims = Claims {
        sub: username.to_string(),
        tutur_id: tutor_id,
        exp: exp.timestamp() as usize,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(JWT_SECRET.as_ref()))
}