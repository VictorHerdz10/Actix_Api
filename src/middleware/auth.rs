use actix_web::{dev::{Service, ServiceRequest, ServiceResponse, Transform}, Error, HttpMessage};
use std::future::{ready, Ready};
use std::pin::Pin;
use std::task::{Context, Poll};

use crate::utils::jwt::validar_token;
use crate::errors::api_error::ApiError;

pub struct Authentication;

impl<S, B> Transform<S, ServiceRequest> for Authentication
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthenticationMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthenticationMiddleware { service }))
    }
}

pub struct AuthenticationMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthenticationMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn std::future::Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // Saltar autenticación para rutas públicas
        let path = req.path();
        
        let rutas_publicas = [
            "/api/salud",
            "/api/info",
            "/api/auth/login",
            "/api/auth/registro"
        ];

        if rutas_publicas.contains(&path) {
            let fut = self.service.call(req);
            return Box::pin(async move {
                let res = fut.await?;
                Ok(res)
            });
        }

         // Extraer y validar token JWT usando la función validar_token
        if let Some(auth_header) = req.headers().get("Authorization") {
            if let Ok(auth_str) = auth_header.to_str() {
                if auth_str.starts_with("Bearer ") {
                    let token = &auth_str[7..];
                    
                    match validar_token(token) {
                        Ok(claims) => {
                            req.extensions_mut().insert(claims);
                            let fut = self.service.call(req);
                            return Box::pin(async move {
                                let res = fut.await?;
                                Ok(res)
                            });
                        }
                        Err(e) => {
                            tracing::warn!("Token inválido: {}", e);
                        }
                    }
                }
            }
        }

        // Retornar no autorizado usando ApiError
        Box::pin(async move {
            let api_error = ApiError::unauthorized("Token inválido o faltante".to_string());
            Err(api_error.into())
        })
    }
}