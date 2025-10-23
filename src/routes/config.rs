use actix_web::web;
use super::user_routes;
use super::auth_routes;
use super::health_routes;

pub fn config_routes(cfg: &mut web::ServiceConfig) {
    cfg
        .configure(health_routes::config)
        .configure(auth_routes::config)
        .configure(user_routes::config);
}