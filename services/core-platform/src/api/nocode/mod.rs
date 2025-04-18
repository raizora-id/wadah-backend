mod schema;
mod ui;
mod workflow;

use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/nocode")
            .configure(schema::configure)
            .configure(ui::configure)
            .configure(workflow::configure)
    );
}
