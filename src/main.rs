mod error;

use error::*;

use std::path::{Path, PathBuf};

use lyweb::*;

use actix_files::Files;
use actix_web::{web, App, http::{header::ContentType, StatusCode}, HttpRequest, HttpResponse, HttpServer};
use reqwest::Url;
use chrono;
use serde_json::Value;

struct NavyData {
    phase: Option<String>,
    fracillum: Option<String>,
    sunrise: Option<String>,
    sunset: Option<String>,
    moonrise: Option<String>,
    moonset: Option<String>
}

impl NavyData {
    fn new() -> Self {
        NavyData {
            phase: None,
            fracillum: None,
            sunrise: None,
            sunset: None,
            moonrise: None,
            moonset: None,
        }
    }
}

async fn get_navy_moon_data() -> Result<NavyData, ApiError> {
    let today = format!("{}", chrono::Local::now().format("%Y-%m-%d"));

    let navy_url = Url::parse_with_params(
        "https://aa.usno.navy.mil/api/rstt/oneday",
        &[
            ("date", today),
            ("coords", "39.90,-75.35".to_string()),
            ("tz", "-4".to_string()),
            ("id", "LYRAPINK".to_string()),
        ],
    )?;

    let response = reqwest::get(navy_url).await?.text().await?;

    let response_json = serde_json::from_str::<Value>(&response)?;

    let mut navy_data = NavyData::new();

    if let Some(properties) = response_json.get("properties") {
        if let Some(data) = properties.get("data") {
            if let Some(curphase) = data.get("curphase") {
                navy_data.phase = Some(curphase.to_string().trim_matches('"').to_string());
            }
            if let Some(fracillum) = data.get("fracillum") {
                navy_data.fracillum = Some(fracillum.to_string().trim_matches('"').to_string());
            }
            if let Some(sundata) = data.get("sundata") {
                if let Some(sundata_vec) = sundata.as_array() {
                    for s in sundata_vec {
                        if let Some(phen) = s.get("phen") &&
                                    let Some(time) = s.get("time") {
                            if phen.to_string() == "\"Rise\"" {
                                navy_data.sunrise = Some(time.to_string().trim_matches('"').to_string());
                            } else if phen.to_string() == "\"Set\"" {
                                navy_data.sunset = Some(time.to_string().trim_matches('"').to_string());
                            }
                        }
                    }
                }
            }
            if let Some(moondata) = data.get("moondata") {
                if let Some(moondata_vec) = moondata.as_array() {
                    for m in moondata_vec {
                        if let Some(phen) = m.get("phen") &&
                                    let Some(time) = m.get("time") {
                            if phen.to_string() == "\"Rise\"" {
                                navy_data.moonrise = Some(time.to_string().trim_matches('"').to_string());
                            } else if phen.to_string() == "\"Set\"" {
                                navy_data.moonset = Some(time.to_string().trim_matches('"').to_string());
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(navy_data)
}

async fn index(_req: HttpRequest) -> Result<HttpResponse, HttpError> {
    let navy_data = get_navy_moon_data().await.unwrap_or(NavyData::new());

    Ok(HttpResponse::build(StatusCode::OK)
        .insert_header(ContentType::html())
        .body(LyWebpage::from_file("www/templates/main.html")?
            .fill_from_file("content", "www/pages/index.html")?
            .resolve_ifs("/")?
            .fill_with_str("phase", &navy_data.phase.unwrap_or("Unknown".to_string()))
            .fill_with_str("fracillum", &navy_data.fracillum.unwrap_or("unknown%".to_string()))
            .fill_with_str("sunrise", &navy_data.sunrise.unwrap_or(" None".to_string()))
            .fill_with_str("sunset", &navy_data.sunset.unwrap_or(" None".to_string()))
            .fill_with_str("moonrise", &navy_data.moonrise.unwrap_or(" None".to_string()))
            .fill_with_str("moonset", &navy_data.moonset.unwrap_or(" None".to_string()))
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
