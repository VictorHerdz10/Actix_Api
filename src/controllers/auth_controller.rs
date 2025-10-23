use actix_web::{web, HttpResponse};
use sea_orm::{DatabaseConnection, EntityTrait, ColumnTrait, QueryFilter};
use crate::models::user::{LoginDto, Entity as UserEntity};
use crate::utils::hash::verify_password;
use crate::utils::jwt::{generar_token};
use crate::errors::api_error::ApiError;
use serde::Serialize;

#[derive(Debug, Serialize)]
struct LoginResponse {
    success: bool,
    message: String,
    token: String,
    usuario: UserInfo,
}

#[derive(Debug, Serialize)]
struct UserInfo {
    id: i32, // Cambiado de String a i32
    email: String,
    nombre: String,
}

pub async fn login(
    db: web::Data<DatabaseConnection>,
    login_data: web::Json<LoginDto>,
) -> Result<HttpResponse, ApiError> {
    // Validar que el email y contraseña no estén vacíos
    if login_data.email.trim().is_empty() || login_data.password.trim().is_empty() {
        return Err(ApiError::bad_request(
            "El email y la contraseña son requeridos".to_string()
        ));
    }

    let usuario = UserEntity::find()
        .filter(crate::models::user::Column::Email.eq(&login_data.email))
        .one(db.get_ref())
        .await
        .map_err(|e| ApiError::internal_server_error(e.to_string()))?;

    match usuario {
        Some(usuario) => {
            // Verificar la contraseña
            if verify_password(&login_data.password, &usuario.password)
                .map_err(|e| ApiError::internal_server_error(e.to_string()))? {
                
                // Generar token JWT (convertir id a String para el token)
                let token = generar_token(usuario.id.to_string())
                    .map_err(|e| ApiError::internal_server_error(
                        format!("Error al generar el token: {}", e)
                    ))?;

                // Crear respuesta
                let respuesta = LoginResponse {
                    success: true,
                    message: "Inicio de sesión exitoso".to_string(),
                    token,
                    usuario: UserInfo {
                        id: usuario.id, // Ahora es i32 directamente
                        email: usuario.email,
                        nombre: usuario.name,
                    },
                };

                Ok(HttpResponse::Ok().json(respuesta))
            } else {
                Err(ApiError::unauthorized("Credenciales inválidas".to_string()))
            }
        }
        None => Err(ApiError::unauthorized("Credenciales inválidas".to_string())),
    }
}