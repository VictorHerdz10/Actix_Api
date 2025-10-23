use actix_web::{web, HttpResponse, Result};
use sea_orm::{DatabaseConnection, EntityTrait, ActiveModelTrait, Set, ModelTrait, IntoActiveModel};
use crate::models::user::{CreateUserDto, UpdateUserDto, Entity as UserEntity, Model as UserModel};
use crate::utils::hash::hash_password;
use crate::errors::api_error::ApiError;
use crate::utils::jwt::Claims;
use chrono::Utc;


pub async fn perfil(
    db: web::Data<DatabaseConnection>,
    claims: web::ReqData<Claims>,
) -> Result<HttpResponse, ApiError> {
    let usuario_id = &claims.sub;
 println!("{}",&usuario_id.to_string());
    let usuario = UserEntity::find_by_id(usuario_id.parse::<i32>()
        .map_err(|_| ApiError::bad_request("ID de usuario inválido".to_string()))?)
        .one(db.get_ref())
        .await
        .map_err(|e| ApiError::internal_server_error(e.to_string()))?;

    match usuario {
        Some(usuario) => {
            Ok(HttpResponse::Ok().json(serde_json::json!({
                "exito": true,
                "datos": {
                    "id": usuario.id,
                    "nombre": usuario.name,
                    "email": usuario.email,
                    "fecha_creacion": usuario.created_at
                },
                "mensaje": "Perfil obtenido exitosamente"
            })))
        }
        None => Err(ApiError::not_found("Usuario no encontrado".to_string())),
    }
}

pub async fn get_users(
    db: web::Data<DatabaseConnection>,
) -> Result<HttpResponse, ApiError> {
    let users = UserEntity::find()
        .all(db.get_ref())
        .await
        .map_err(|e| ApiError::internal_server_error(e.to_string()))?;

    Ok(HttpResponse::Ok().json(users))
}

pub async fn get_user(
    db: web::Data<DatabaseConnection>,
    id: web::Path<i32>,
) -> Result<HttpResponse, ApiError> {
    let user = UserEntity::find_by_id(*id)
        .one(db.get_ref())
        .await
        .map_err(|e| ApiError::internal_server_error(e.to_string()))?;

    match user {
        Some(user) => Ok(HttpResponse::Ok().json(user)),
        None => Err(ApiError::not_found("Usuario no encontrado".to_string())),
    }
}

pub async fn create_user(
    db: web::Data<DatabaseConnection>,
    user_data: web::Json<CreateUserDto>,
) -> Result<HttpResponse, ApiError> {
    let hashed_password = hash_password(&user_data.password)
        .map_err(|e| ApiError::internal_server_error(e.to_string()))?;

    let user = crate::models::user::ActiveModel {
        name: Set(user_data.name.clone()),
        email: Set(user_data.email.clone()),
        password: Set(hashed_password),
        created_at: Set(Utc::now()),
        updated_at: Set(Utc::now()),
        ..Default::default()
    };

    let user: UserModel = user.insert(db.get_ref())
        .await
        .map_err(|e| ApiError::internal_server_error(e.to_string()))?;

    Ok(HttpResponse::Created().json(user))
}

pub async fn update_user(
    db: web::Data<DatabaseConnection>,
    id: web::Path<i32>,
    user_data: web::Json<UpdateUserDto>,
) -> Result<HttpResponse, ApiError> {
    let user = UserEntity::find_by_id(*id)
        .one(db.get_ref())
        .await
        .map_err(|e| ApiError::internal_server_error(e.to_string()))?;

    let mut user = match user {
        Some(user) => user.into_active_model(),
        None => return Err(ApiError::not_found("Usuario no encontrado".to_string())),
    };

    if let Some(name) = &user_data.name {
        user.name = Set(name.clone());
    }
    
    if let Some(email) = &user_data.email {
        user.email = Set(email.clone());
    }
    
    if let Some(password) = &user_data.password {
        let hashed_password = hash_password(password)
            .map_err(|e| ApiError::internal_server_error(e.to_string()))?;
        user.password = Set(hashed_password);
    }

    user.updated_at = Set(Utc::now());

    let user: UserModel = user.update(db.get_ref())
        .await
        .map_err(|e| ApiError::internal_server_error(e.to_string()))?;

    Ok(HttpResponse::Ok().json(user))
}

pub async fn delete_user(
    db: web::Data<DatabaseConnection>,
    id: web::Path<i32>,
) -> Result<HttpResponse, ApiError> {
    let user = UserEntity::find_by_id(*id)
        .one(db.get_ref())
        .await
        .map_err(|e| ApiError::internal_server_error(e.to_string()))?;

    match user {
        Some(user) => {
            user.delete(db.get_ref())
                .await
                .map_err(|e| ApiError::internal_server_error(e.to_string()))?;
            
            Ok(HttpResponse::Ok().json(serde_json::json!({
                "message": "Usuario eliminado exitoxamente"
            })))
        }
        None => Err(ApiError::not_found("Usuario no encontrado".to_string())),
    }
}