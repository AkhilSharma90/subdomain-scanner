use super::loader::read_file;
use actix_web::HttpResponse;

pub async fn main() -> HttpResponse {
    let mut html = read_file(String::from("./app/main.html"));

    let javascript = read_file(String::from("./app/app.js"));

    let main_css = read_file(String::from("./app/styles/main.css"));

    let header_css = read_file(String::from("./app/styles/header.css"));

    html = html
        .replace("{{JAVASCRIPT}}", &javascript)
        .replace("{{MAIN_CSS}}", &main_css)
        .replace("{{HEADER_CSS}}", &header_css);

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}
