use actix_web::{App, HttpServer};

mod core;
mod json_serialization;
mod views;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let address = "127.0.0.1:8000";
    println!("Server configured to run at http://{}", address);

    HttpServer::new(move || {
        App::new().configure(views::views_factory)
    })
    .bind(address)?
    .run()
    .await
}
