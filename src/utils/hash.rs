use bcrypt::{hash, verify, DEFAULT_COST};

pub fn hash_password(contraseña: &str) -> Result<String, bcrypt::BcryptError> {
    let cost: u32 = std::env::var("BCRYPT_COST")
        .unwrap_or_else(|_| DEFAULT_COST.to_string())
        .parse()
        .unwrap_or(DEFAULT_COST);
    
    hash(contraseña, cost)
}

pub fn verify_password(contraseña: &str, hasheada: &str) -> Result<bool, bcrypt::BcryptError> {
    verify(contraseña, hasheada)
}