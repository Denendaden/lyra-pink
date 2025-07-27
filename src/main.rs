mod error;

use error::*;

use std::env;

use lyssg::ssg::*;

use actix_files::Files;
use actix_web::{web, App, http::{header::ContentType, StatusCode}, HttpRequest, HttpResponse, HttpServer};

async fn index(_req: HttpRequest) -> Result<HttpResponse, HttpError> {
    Ok(HttpResponse::build(StatusCode::OK)
        .insert_header(ContentType::html())
        .body(LyWebpage::from_file("templates/main.html")?
            .fill_from_file("content", "www/index.html")?
            .resolve_ifs("/")?
            // add status information
            .fill_from_file(
                "status",
                env::var("STATUS_FILE").unwrap_or("www/status.md".to_string()),
            )?
            .contents
        )
    )
}

async fn load_page(req: HttpRequest) -> Result<HttpResponse, HttpError> {
    let path = req.match_info().query("path");
    let content_path = "www/".to_string() + path + ".html";

    Ok(HttpResponse::build(StatusCode::OK)
        .insert_header(ContentType::html())
        .body(LyWebpage::from_file("templates/main.html")?
            .fill_from_file("content", content_path)?
            .resolve_ifs(path)?
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
