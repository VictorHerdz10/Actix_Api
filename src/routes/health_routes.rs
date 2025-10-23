use crate::controllers::{health_controller, user_controller};
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("/api/salud", web::get().to(health_controller::health_check))
        .route("/api/perfil", web::get().to(user_controller::perfil))
        .route("/api/info", web::get().to(health_controller::api_info));
}
