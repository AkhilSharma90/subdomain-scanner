mod loader;
mod main;

use super::path::Path;
use actix_web::web;

pub fn app_factory(app: &mut web::ServiceConfig) {
    let base_path: Path = Path {
        prefix: String::from("/"),
        backend: false,
    };
    app.route(
        &base_path.define(String::from("")),
        web::get().to(main::main),
    );
}
