mod config;
mod controllers;
mod errors;
mod middleware;
mod models;
mod routes;
mod utils;

use actix_web::{App, HttpServer, web::Data};
use tracing_subscriber::prelude::*;

use crate::middleware::cors::cors_config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Cargar variables de entorno desde .env
    dotenvy::dotenv().ok();
    
    // Inicializar tracing con formato personalizado
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(
            tracing_subscriber::fmt::layer()
                .compact()
                .without_time()
                .with_target(false)
                .with_level(true)
        )
        .init();

    // Inicializar conexión a la base de datos
    let db = match config::database::connect().await {
        Ok(db) => {
            tracing::info!("Conectado a la base de datos exitosamente");
            db
        },
        Err(e) => {
            tracing::error!("Error al conectar con la base de datos: {}", e);
            std::process::exit(1);
        }
    };

    // Obtener puerto del entorno o usar 8080 por defecto
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .expect("PORT debe ser un número");

    let addr = format!("0.0.0.0:{}", port);
    tracing::info!("Iniciando servidor Actix-web en {}", addr);

      // Crear servidor HTTP con CORS configurado correctamente
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(db.clone()))
            .wrap(cors_config())
            .wrap(actix_web::middleware::Logger::default())
            .wrap(middleware::auth::Authentication)
            .configure(routes::config::config_routes)
    })
    .bind(&addr)?
    .run()
    .await
}