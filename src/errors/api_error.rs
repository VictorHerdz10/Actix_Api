use actix_web::{HttpResponse, ResponseError};
use std::fmt;

#[derive(Debug)]
pub struct ApiError {
    pub mensaje: String,
    pub codigo_estado: u16,
}

impl ApiError {
    pub fn new(mensaje: String, codigo_estado: u16) -> Self {
        Self { mensaje, codigo_estado }
    }

    pub fn internal_server_error(mensaje: String) -> Self {
        Self::new(mensaje, 500)
    }

    pub fn not_found(mensaje: String) -> Self {
        Self::new(mensaje, 404)
    }

    pub fn bad_request(mensaje: String) -> Self {
        Self::new(mensaje, 400)
    }

    pub fn unauthorized(mensaje: String) -> Self {
        Self::new(mensaje, 401)
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.mensaje)
    }
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        let status = actix_web::http::StatusCode::from_u16(self.codigo_estado)
            .unwrap_or(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR);

        HttpResponse::build(status).json(serde_json::json!({
            "success": false,
            "message": self.mensaje,
            "status_code": self.codigo_estado
        }))
    }
}