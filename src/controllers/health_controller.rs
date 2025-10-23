use actix_web::HttpResponse;

pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "estado": "OK",
        "mensaje": "Servidor funcionando correctamente"
    }))
}

pub async fn api_info() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "nombre": "API Rust con Actix-web",
        "version": "0.1.0",
        "descripcion": "Una API REST simple construida con Rust y el framework Actix-web",
        "framework": "Actix-web",
        "arquitectura": "MVC",
        "endpoints": {
            "GET /api/salud": "Verificación del estado del servidor",
            "GET /api/info": "Información de la API",
            "POST /api/auth/registro": "Registrar nuevo usuario",
            "POST /api/auth/login": "Iniciar sesión",
            "GET /api/usuarios": "Obtener todos los usuarios (protegido)",
            "GET /api/usuarios/{id}": "Obtener usuario por ID (protegido)",
            "PUT /api/usuarios/{id}": "Actualizar usuario (protegido)",
            "DELETE /api/usuarios/{id}": "Eliminar usuario (protegido)"
        }
    }))
}