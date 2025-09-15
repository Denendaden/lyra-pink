mod error;

use error::*;

use std::path::{Path, PathBuf};

use lyweb::*;

use actix_files::Files;
use actix_web::{web, App, http::{header::ContentType, StatusCode}, HttpRequest, HttpResponse, HttpServer};

async fn index(_req: HttpRequest) -> Result<HttpResponse, HttpError> {
    Ok(HttpResponse::build(StatusCode::OK)
        .insert_header(ContentType::html())
        .body(LyWebpage::from_file("www/templates/main.html")?
            .fill_from_file("content", "www/pages/index.html")?
            .resolve_ifs("/")?
            .contents
        )
    )
}

async fn photos(_req: HttpRequest) -> Result<HttpResponse, HttpError> {
    let mut webpage = LyWebpage::from_file("www/templates/main.html")?
        .fill_from_file("content", "www/pages/photos.html")?
        .resolve_ifs("photos")?;

    let photos_dir = Path::new("www/static/photos");
    if let Ok(d) = photos_dir.read_dir() {
        for shoot in d {
            if let Ok(s) = shoot {
                if s.path().is_dir() {
                    let mut gallery_html = "<div class=\"photo-gallery\">".to_string();
                    if let Ok(photos) = s.path().read_dir() {
                        for photo in photos {
                            if let Ok(p) = photo {
                                // remove first part of path (the www part)
                                let path = PathBuf::from_iter(p.path().components().skip(1));
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
    let content_path = "www/pages/".to_string() + path + ".html";

    Ok(HttpResponse::build(StatusCode::OK)
        .insert_header(ContentType::html())
        .body(LyWebpage::from_file("www/templates/main.html")?
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
            .service(Files::new("/static", "www/static"))
    })
        .bind(("0.0.0.0", 5566))?
        .run()
        .await
}
