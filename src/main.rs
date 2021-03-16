use std::{fs, collections::HashMap, io};
use actix_web::{web, middleware::Logger, App, HttpServer, HttpResponse};
use mongodb::{Client, Database};
use tera::Tera;

mod api;
mod r#static;
mod util;

struct AppData {
  db: Database,
  tera: Tera,
  files: HashMap<String, Vec<u8>>,
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

fn load_static() -> io::Result<HashMap<String, Vec<u8>>> {
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

#[actix_web::main]
async fn main() -> io::Result<()> {
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

  let files = load_static()?;

  let app = AppData { db, tera, files };
  let app_ref = web::Data::new(app);

  HttpServer::new(move || {
    App::new()
      .app_data(app_ref.clone())
      .wrap(Logger::default())
      .route("/{file}.css", web::get().to(handle_css))
      .route("/{file}.js", web::get().to(handle_js))
      .service(web::scope("/api").configure(api::config))
      .service(web::scope("/").configure(r#static::config))
  })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}
