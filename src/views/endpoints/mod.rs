pub mod about;
pub mod help;
pub mod scan;

use super::path::Path;
use actix_web::web;

pub fn endpoint_factory(app: &mut web::ServiceConfig) {
    let base_path = Path {
        prefix: String::from(""),
        backend: true,
    };

    app.route(
        &base_path.define(String::from("/scan/{domain}")),
        web::get().to(scan::all::scan),
    )
    .route(
        &base_path.define(String::from("/about")),
        web::get().to(about::about),
    )
    .route(
        &base_path.define(String::from("/help")),
        web::get().to(help::help),
    )
    .route(
        &base_path.define(String::from("/scanners")),
        web::get().to(about::list_scanners),
    );
}
