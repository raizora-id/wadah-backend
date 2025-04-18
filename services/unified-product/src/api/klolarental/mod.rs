mod vehicle;
mod booking;
mod tour;

use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/klolarental")
            .configure(vehicle::configure)
            .configure(booking::configure)
            .configure(tour::configure)
    );
}