#[macro_use]
extern crate serde_json;

use actix_web::{web, App, HttpServer};
use mongodb::{Client, Database};
use handlebars::Handlebars;

mod api;
mod r#static;
mod util;

struct AppData<'a> {
  db: Database,
  hb: Handlebars<'a>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  let mongodb_uri = match std::env::var("MONGODB_URI") {
    Ok(val) => val,
    Err(_) => "mongodb://localhost:27017".into(),
  };
  let client = Client::with_uri_str(&mongodb_uri).await.unwrap();
  let db = client.database("medialibre");

  let mut hb = Handlebars::new();
  hb
    .register_templates_directory(".hbs", "./static/templates")
    .unwrap();

  let app = AppData { db, hb };
  let app_ref = web::Data::new(app);

  HttpServer::new(move || {
    App::new()
      .app_data(app_ref.clone())
      .service(web::scope("/api").configure(api::config))
      .service(web::scope("/").configure(r#static::config))
  })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}
