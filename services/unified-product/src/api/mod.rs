mod klolatoko;
mod klolakos;
mod klolarental;
mod klolaform;

use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .configure(klolatoko::configure)
            .configure(klolakos::configure)
            .configure(klolarental::configure)
            .configure(klolaform::configure)
    );
}