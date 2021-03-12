use actix_web::{web, App, HttpServer};
use mongodb::{Client, options::ClientOptions};

mod api;
mod util;

struct State {
  mongo: mongodb::Database,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  let mongodb_uri = match std::env::var("MONGODB_URI") {
    Ok(val) => val,
    Err(_) => "mongodb://localhost:27017".into(),
  };
  let client_options = ClientOptions::parse(&mongodb_uri).await.unwrap();
  let client = Client::with_options(client_options).unwrap();

  HttpServer::new(move || {
    App::new()
      .data(State {
        mongo: client.database("medialibre"),
      })
      .service(web::scope("/api").configure(api::config))
  })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}
