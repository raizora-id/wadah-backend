mod property;
mod unit;
mod tenant;

use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/klolakos")
            .configure(property::configure)
            .configure(unit::configure)
            .configure(tenant::configure)
    );
}