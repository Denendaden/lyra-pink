use lyssg::{error::*, ssg::*};

use std::fs;

use actix_files::Files;
use actix_web::{web, App, http::{header::ContentType, StatusCode}, HttpRequest, HttpResponse, HttpServer};

async fn index(_req: HttpRequest) -> Result<HttpResponse, LyError> {
    Ok(HttpResponse::build(StatusCode::OK)
        .insert_header(ContentType::html())
        .body(LyWebpage::read_file("templates/template.html")?
            .fill_template("content", &fs::read_to_string("www/index.html")?)
            .contents
        )
    )
}

async fn load_page(req: HttpRequest) -> Result<HttpResponse, LyError> {
    let content_path = "www/".to_string() + req.match_info().query("path") + ".html";

    Ok(HttpResponse::build(StatusCode::OK)
        .insert_header(ContentType::html())
        .body(LyWebpage::read_file("templates/template.html")?
            .fill_template("content", &fs::read_to_string(content_path)?)
            .contents
        )
    )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/{path}", web::get().to(load_page))
            .service(Files::new("/static", "static"))
    })
        .bind(("127.0.0.1", 8000))?
        .run()
        .await
}
