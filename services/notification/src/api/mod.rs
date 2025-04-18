mod notification;
mod template;
mod preference;

use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .configure(notification::configure)
            .configure(template::configure)
            .configure(preference::configure)
    );
}