use crate::{AppData, util::languages, api::music::artist::Artist};
use actix_web::{web, HttpResponse};
use actix_session::Session;
use mongodb::bson::{doc, from_bson, oid::ObjectId, Bson};
use tera::Context;

async fn new_artist(app: web::Data<AppData>, session: Session) -> HttpResponse {
  let lang = session.get::<String>("lang").unwrap().unwrap_or("en".into());

  let mut context = Context::new();
  context.insert("media", "music");
  context.insert("lang", &lang);
  context.insert("languages", &languages());
  context.insert("locale", &app.locales.get(&lang).unwrap());
  let body = app.tera.render("music/new/artist.html", &context).unwrap();

  HttpResponse::Ok().body(body)
}

async fn artist(app: web::Data<AppData>, session: Session, web::Path(artist_id): web::Path<String>) -> HttpResponse {
  let lang = session.get::<String>("lang").unwrap().unwrap_or("en".into());
  
  let artists = app.db.collection("music_artists");
  let id = ObjectId::with_string(&artist_id).unwrap();
  let artist_data = artists.find_one(doc! { "_id": id }, None).await.unwrap().unwrap();
  let artist: Artist = from_bson(Bson::Document(artist_data)).unwrap();

  let mut context = Context::new();
  context.insert("media", "music");
  context.insert("lang", &lang);
  context.insert("languages", &languages());
  context.insert("locale", &app.locales.get(&lang).unwrap());
  context.insert("name", artist.name.locale_names.get(&lang).unwrap());
  let body = app.tera.render("music/artist.html", &context).unwrap();

  HttpResponse::Ok().body(body)
}

pub fn config(cfg: &mut web::ServiceConfig) {
  cfg
    .route("/artist/new", web::get().to(new_artist))
    .route("/artist/{artist_id}", web::get().to(artist));
}
