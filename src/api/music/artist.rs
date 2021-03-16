use crate::{AppData, util::language::Language};
use std::collections::HashMap;
use actix_web::{web, HttpResponse};
use serde::{Serialize, Deserialize};
use mongodb::bson::{self, doc, oid::ObjectId};

#[derive(Serialize, Deserialize)]
pub struct ArtistName {
  pub locale_names: HashMap<Language, String>,
  pub first_name: Option<String>,
  pub last_name: Option<String>,
  pub middle_name: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Date {
  pub year: Option<u32>,
  pub month: Option<u32>,
  pub day: Option<u32>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Location {
  pub country: Option<String>,
  pub state: Option<String>,
  pub city: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Artist {
  pub group: bool,
  pub name: ArtistName,
  pub birth: Date,
  pub death: Date,
  pub origin_location: Location,
  pub current_location: Location,
  pub members: Vec<String>,
  pub bio: HashMap<Language, String>,
  pub images: Vec<String>,
  pub website: Option<String>,
}

#[derive(Deserialize)]
struct CreateArtist {
  pub name: String,
}

async fn create_artist(app: web::Data<AppData<'_>>, form: web::Form<CreateArtist>) -> HttpResponse {
  let mut locale_names = HashMap::new();
  locale_names.insert(Language::EN, form.name.clone());
  let name = ArtistName {
    locale_names: locale_names,
    first_name: None,
    last_name: None,
    middle_name: None,
  };

  let mut bio = HashMap::new();
  bio.insert(Language::EN, "".into());

  let date = Date {
    year: None,
    month: None,
    day: None,
  };

  let loc = Location {
    country: None,
    state: None,
    city: None,
  };

  let artist = Artist {
    group: true,
    name,
    birth: date.clone(),
    death: date,
    origin_location: loc.clone(),
    current_location: loc,
    members: Vec::new(),
    bio,
    images: Vec::new(),
    website: None,
  };

  let artists = app.db.collection("music_artists");
  let res = artists.insert_one(bson::to_document(&artist).unwrap(), None).await.unwrap();
  let id = res.inserted_id.as_object_id().unwrap().to_hex();

  HttpResponse::SeeOther()
    .header("Location", format!("/music/artist/{}", id))
    .body(id)
}

async fn get_artist(app: web::Data<AppData<'_>>, web::Path(artist_id): web::Path<String>) -> HttpResponse {
  let artists = app.db.collection("music_artists");
  let id = ObjectId::with_string(&artist_id).unwrap();
  let artist_data = artists.find_one(doc! { "_id": id }, None).await.unwrap().unwrap();
  let artist: Artist = bson::from_document(artist_data).unwrap();

  HttpResponse::Ok().json(artist)
}

pub fn config(cfg: &mut web::ServiceConfig) {
  cfg
    .route("", web::get().to(|| HttpResponse::Ok()))
    .route("", web::post().to(create_artist))
    .service(
      web::resource("/{artist_id}")
        .route(web::get().to(get_artist))
    );
}
