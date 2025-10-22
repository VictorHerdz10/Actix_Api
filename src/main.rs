use actix_cors::Cors;
use actix_web::{
    App, HttpServer, Result, delete, get,
    middleware::Logger,
    post, put,
    web::{Data, Json, Path, Query},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tracing_subscriber::prelude::*;

type AppState = Data<Arc<Mutex<HashMap<String, User>>>>;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct User {
    id: String,
    name: String,
    email: String,
    created_at: String,
}

#[derive(Debug, Deserialize)]
struct CreateUser {
    name: String,
    email: String,
}

#[derive(Debug, Deserialize)]
struct UpdateUser {
    name: Option<String>,
    email: Option<String>,
}

#[derive(Debug, Deserialize)]
struct QueryParams {
    limit: Option<usize>,
    offset: Option<usize>,
}

#[derive(Debug, Serialize)]
struct ApiResponse<T> {
    success: bool,
    data: Option<T>,
    message: String,
}

impl<T> ApiResponse<T> {
    fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            message: "Operation successful".to_string(),
        }
    }

    fn error(message: String) -> Self {
        Self {
            success: false,
            data: None,
            message,
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Initialize state
    let state: Arc<Mutex<HashMap<String, User>>> = Arc::new(Mutex::new(HashMap::new()));
    let app_state = Data::new(state);

    // Get port from environment or default to 8080 (Vercel usa 8080)
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .expect("PORT must be a number");

    let addr = format!("0.0.0.0:{}", port);
    tracing::info!("Starting Actix-web server on {}", addr);

    // Create HTTP server
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .app_data(app_state.clone())
            .wrap(cors)
            .wrap(Logger::default())
            .service(health_check)
            .service(api_info)
            .service(get_users)
            .service(get_user)
            .service(create_user)
            .service(update_user)
            .service(delete_user)
    })
    .bind(&addr)?
    .run()
    .await
}

#[get("/health")]
async fn health_check() -> Result<Json<ApiResponse<String>>> {
    Ok(Json(ApiResponse::success("OK".to_string())))
}

#[get("/")]
async fn api_info() -> Result<Json<ApiResponse<serde_json::Value>>> {
    let info = serde_json::json!({
        "name": "Rust API with Actix-web",
        "version": "0.1.0",
        "description": "A simple REST API built with Rust and Actix-web framework",
        "framework": "Actix-web",
        "endpoints": {
            "GET /health": "Health check endpoint",
            "GET /": "API information",
            "GET /api/users": "Get all users",
            "POST /api/users": "Create a new user",
            "GET /api/users/{id}": "Get user by ID",
            "PUT /api/users/{id}": "Update user by ID",
            "DELETE /api/users/{id}": "Delete user by ID"
        }
    });

    Ok(Json(ApiResponse::success(info)))
}

#[get("/api/users")]
async fn get_users(
    state: AppState,
    query: Query<QueryParams>,
) -> Result<Json<ApiResponse<Vec<User>>>> {
    let users = state.lock().unwrap();
    let mut users_vec: Vec<User> = users.values().cloned().collect();

    // Sort by created_at
    users_vec.sort_by(|a, b| a.created_at.cmp(&b.created_at));

    // Apply pagination
    let offset = query.offset.unwrap_or(0);
    let limit = query.limit.unwrap_or(users_vec.len());

    let paginated_users = users_vec.into_iter().skip(offset).take(limit).collect();

    Ok(Json(ApiResponse::success(paginated_users)))
}

#[get("/api/users/{id}")]
async fn get_user(state: AppState, path: Path<String>) -> Result<Json<ApiResponse<User>>> {
    let id = path.into_inner();
    let users = state.lock().unwrap();

    match users.get(&id) {
        Some(user) => Ok(Json(ApiResponse::success(user.clone()))),
        None => Ok(Json(ApiResponse::error("User not found".to_string()))),
    }
}

#[post("/api/users")]
async fn create_user(
    state: AppState,
    user_data: Json<CreateUser>,
) -> Result<Json<ApiResponse<User>>> {
    let id = uuid::Uuid::new_v4().to_string();
    let created_at = chrono::Utc::now().to_rfc3339();

    let user = User {
        id: id.clone(),
        name: user_data.name.clone(),
        email: user_data.email.clone(),
        created_at,
    };

    state.lock().unwrap().insert(id, user.clone());
    Ok(Json(ApiResponse::success(user)))
}

#[put("/api/users/{id}")]
async fn update_user(
    state: AppState,
    path: Path<String>,
    user_data: Json<UpdateUser>,
) -> Result<Json<ApiResponse<User>>> {
    let id = path.into_inner();
    let mut users = state.lock().unwrap();

    match users.get_mut(&id) {
        Some(user) => {
            if let Some(name) = &user_data.name {
                user.name = name.clone();
            }
            if let Some(email) = &user_data.email {
                user.email = email.clone();
            }
            Ok(Json(ApiResponse::success(user.clone())))
        }
        None => Ok(Json(ApiResponse::error("User not found".to_string()))),
    }
}

#[delete("/api/users/{id}")]
async fn delete_user(state: AppState, path: Path<String>) -> Result<Json<ApiResponse<String>>> {
    let id = path.into_inner();
    let mut users = state.lock().unwrap();

    match users.remove(&id) {
        Some(_) => Ok(Json(ApiResponse::success(
            "User deleted successfully".to_string(),
        ))),
        None => Ok(Json(ApiResponse::error("User not found".to_string()))),
    }
}