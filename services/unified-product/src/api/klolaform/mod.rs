mod form;
mod response;

use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/klolaform")
            .configure(form::configure)
            .configure(response::configure)
    );
}