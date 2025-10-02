use actix_web::web::to;
use serde::{Serialize, Deserialize};
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};

/// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
#[derive(Debug, Clone, Serialize, Deserialize)]
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

pub fn validate_jwt(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let token_data = decode::<Claims>(&token, &DecodingKey::from_secret(JWT_SECRET.as_ref()), &Validation::default())?;
    Ok(token_data.claims)
}

pub fn get_username_from_token(token: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let claims = validate_jwt(token)?;
    Ok(claims.sub)
}