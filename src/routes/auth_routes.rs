use actix_web::web;
use crate::controllers::auth_controller;
use crate::controllers::user_controller;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/auth")
            .route("/login", web::post().to(auth_controller::login))
            .route("/registro", web::post().to(user_controller::create_user))
    );
}