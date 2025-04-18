mod product;
mod inventory;
mod sales;

use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/klolatoko")
            .configure(product::configure)
            .configure(inventory::configure)
            .configure(sales::configure)
    );
}