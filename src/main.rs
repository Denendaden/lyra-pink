mod error;

use error::*;

use std::{env, path::Path};

use lyweb::*;

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

async fn photos(_req: HttpRequest) -> Result<HttpResponse, HttpError> {
    let mut webpage = LyWebpage::from_file("templates/main.html")?
        .fill_from_file("content", "www/photos.html")?
        .resolve_ifs("photos")?;

    let photos_dir = Path::new("static/photos");
    if let Ok(d) = photos_dir.read_dir() {
        for shoot in d {
            if let Ok(s) = shoot {
                if s.path().is_dir() {
                    let mut gallery_html = "<div class=\"photo-gallery\">".to_string();
                    if let Ok(photos) = s.path().read_dir() {
                        for photo in photos {
                            if let Ok(p) = photo {
                                let path = p.path();
                                let spath = path.to_string_lossy();
                                gallery_html += &format!("<img src=\"{spath}\">").to_string();
                            }
                        }
                    }
                    gallery_html += "</div>";
                    webpage = webpage.fill_with_str(&s.file_name().to_string_lossy(), &gallery_html);
                }
            }
        }
    }

    Ok(HttpResponse::build(StatusCode::OK)
        .insert_header(ContentType::html())
        .body(webpage.contents)
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
            .route("/photos", web::get().to(photos))
            .route("/{path}", web::get().to(load_page))
            .service(Files::new("/static", "static"))
    })
        .bind(("127.0.0.1", 8000))?
        .run()
        .await
}
