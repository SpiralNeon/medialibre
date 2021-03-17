use std::{fs, collections::HashMap, io};
use serde::{Serialize, Deserialize};
use env_logger::Env;
use actix_web::{web, middleware::Logger, App, HttpServer, HttpResponse};
use actix_session::CookieSession;
use mongodb::{Client, Database};
use tera::Tera;

mod api;
mod pages;
mod util;

#[derive(Serialize, Deserialize)]
struct Link {
  media: String,
  name: String,
}

#[derive(Serialize, Deserialize)]
pub struct Language {
  short: String,
  long: String,
}

#[derive(Serialize, Deserialize)]
struct Locale {
  links: Vec<Link>,
  languages: Vec<Language>,
  category: HashMap<String, String>,
  heading: HashMap<String, String>,
  text: HashMap<String, String>,
}

struct AppData {
  db: Database,
  tera: Tera,
  files: HashMap<String, Vec<u8>>,
  locales: HashMap<String, Locale>,
}

async fn handle_css(app: web::Data<AppData>, file: web::Path<String>) -> HttpResponse {
  let data = app.files.get(&format!("{}.css.gz", file)).unwrap().clone();

  HttpResponse::Ok()
    .content_type("text/css")
    .header("content-encoding", "gzip")
    .body(data)
}

async fn handle_js(app: web::Data<AppData>, file: web::Path<String>) -> HttpResponse {
  let data = app.files.get(&format!("{}.js.gz", file)).unwrap().clone();

  HttpResponse::Ok()
    .content_type("text/javascript")
    .header("content-encoding", "gzip")
    .body(data)
}

fn load_files() -> io::Result<HashMap<String, Vec<u8>>> {
  let mut files = HashMap::new();

  for entry in fs::read_dir("static/build")? {
    let entry = entry?;
    let path = entry.path();
    let file_name = path.file_name().unwrap().to_str().unwrap().to_string();
    if !path.is_dir() {
      let data = fs::read(path).unwrap();
      files.insert(file_name, data);
    }
  }
  
  Ok(files)
}

fn load_locales() -> io::Result<HashMap<String, Locale>> {
  let mut locales = HashMap::new();

  for entry in fs::read_dir("static/locales")? {
    let entry = entry?;
    let path = entry.path();
    let file_stem = path.file_stem().unwrap().to_str().unwrap().to_string();
    if !path.is_dir() {
      let data = fs::read_to_string(path).unwrap();
      let locale = ron::from_str(&data).unwrap();
      locales.insert(file_stem, locale);
    }
  }
  
  Ok(locales)
}

#[actix_web::main]
async fn main() -> io::Result<()> {
  env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

  let mongodb_uri = match std::env::var("MONGODB_URI") {
    Ok(val) => val,
    Err(_) => "mongodb://localhost:27017".into(),
  };
  let client = Client::with_uri_str(&mongodb_uri).await.unwrap();
  let db = client.database("medialibre");

  let tera = match Tera::new("static/templates/**/*.html") {
    Ok(t) => t,
    Err(e) => {
      println!("Parsing error: {}", e);
      std::process::exit(1);
    }
  };

  let files = load_files()?;
  let locales = load_locales()?;

  let app = AppData { db, tera, files, locales };
  let app_ref = web::Data::new(app);

  HttpServer::new(move || {
    App::new()
      .app_data(app_ref.clone())
      .wrap(Logger::default())
      .wrap(
        CookieSession::signed(&[0; 32])
          .name("actix_session")
          .path("/")
          .secure(true)
      )
      .route("/{file}.css", web::get().to(handle_css))
      .route("/{file}.js", web::get().to(handle_js))
      .service(web::scope("/api").configure(api::config))
      .service(web::scope("/").configure(pages::config))
  })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}
