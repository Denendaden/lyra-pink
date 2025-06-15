use lyra_pink::error::*;

use std::path::PathBuf;

use actix_files::{Files, NamedFile};
use actix_web::{web, App, HttpRequest, HttpServer,
};

async fn index(_req: HttpRequest) -> Result<NamedFile, LyError> {
    Ok(NamedFile::open("www/index.html")?)
}

async fn load_page(req: HttpRequest) -> Result<NamedFile, LyError> {
    let path = "www/".to_string() + req.match_info().query("path");
    let path_buf: PathBuf = (path + ".html").parse().unwrap();

    Ok(NamedFile::open(path_buf)?)
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
