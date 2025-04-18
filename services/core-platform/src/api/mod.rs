mod auth;
mod tenant;
mod user;
mod subscription;
mod nocode;

use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .configure(auth::configure)
            .configure(tenant::configure)
            .configure(user::configure)
            .configure(subscription::configure)
            .configure(nocode::configure)
    );
}
