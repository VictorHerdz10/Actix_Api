use jsonwebtoken::{encode, decode, Header, Validation, Algorithm, EncodingKey, DecodingKey};
use serde::{Deserialize, Serialize};
use chrono::{Utc, Duration};

#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

pub fn generar_token(id_usuario: String) -> Result<String, jsonwebtoken::errors::Error> {
    let expiracion_horas: u64 = std::env::var("JWT_EXPIRATION_HOURS")
        .unwrap_or_else(|_| "24".to_string())
        .parse()
        .unwrap_or(24);
    
    let expiracion = Utc::now()
        .checked_add_signed(Duration::hours(expiracion_horas as i64))
        .expect("Tiempo de expiración inválido")
        .timestamp() as usize;

    let claims = Claims {
        sub: id_usuario,
        exp: expiracion,
    };

    let secret = std::env::var("JWT_SECRET")
        .expect("❌ JWT_SECRET debe estar configurada en el archivo .env");
    
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
}

pub fn validar_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let secret = std::env::var("JWT_SECRET")
        .expect("❌ JWT_SECRET debe estar configurada en el archivo .env");
    
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::new(Algorithm::HS256),
    ).map(|data| data.claims)
}