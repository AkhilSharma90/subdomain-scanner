mod app;
mod endpoints;
mod path;

use actix_web::web;

pub fn views_factory(app: &mut web::ServiceConfig) {
    endpoints::endpoint_factory(app);
    app::app_factory(app);
}
