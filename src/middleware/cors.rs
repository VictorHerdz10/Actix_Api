use actix_cors::Cors;

pub fn cors_config() -> Cors {
    Cors::default()
        .allow_any_origin()           // Cualquier origen en desarrollo
        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS", "PATCH"])
        .allow_any_header()           // Cualquier header
        .supports_credentials()       // Permitir credenciales
        .max_age(3600)               // Cache de 1 hora
}