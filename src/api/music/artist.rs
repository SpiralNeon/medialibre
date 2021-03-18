use crate::AppData;
use std::collections::HashMap;
use actix_web::{web, HttpResponse};
use serde::{Serialize, Deserialize};
use mongodb::bson::{self, doc, oid::ObjectId};

#[derive(Serialize, Deserialize)]
pub struct ArtistName {
  pub locale_names: HashMap<String, String>,
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
  pub bio: HashMap<String, String>,
  pub images: Vec<String>,
  pub website: Option<String>,
}

#[derive(Deserialize)]
struct CreateArtist {
  pub r#type: String,
  pub name: String,
  pub first_name: String,
  pub last_name: String,
  pub middle_name: String,
  pub birth_year: u32,
  pub birth_month: u32,
  pub birth_day: u32,
  pub death_year: u32,
  pub death_month: u32,
  pub death_day: u32,
  pub origin_country: String,
  pub origin_state: String,
  pub origin_city: String,
  pub current_country: String,
  pub current_state: String,
  pub current_city: String,
}

async fn create_artist(app: web::Data<AppData>, form: web::Form<CreateArtist>) -> HttpResponse {
  let group = form.r#type == "group";

  let mut locale_names = HashMap::new();
  locale_names.insert("en".into(), form.name.clone());

  let first_name = if form.first_name.is_empty() { None } else { Some(form.first_name.clone()) };
  let last_name = if form.last_name.is_empty() { None } else { Some(form.last_name.clone()) };
  let middle_name = if form.middle_name.is_empty() { None } else { Some(form.middle_name.clone()) };

  let name = ArtistName {
    locale_names,
    first_name,
    last_name,
    middle_name,
  };

  let birth = Date {
    year: if form.birth_year > 0 { Some(form.birth_year) } else { None },
    month: if form.birth_month > 0 { Some(form.birth_month) } else { None },
    day: if form.birth_day > 0 { Some(form.birth_day) } else { None },
  };

  let death = Date {
    year: if form.death_year > 0 { Some(form.death_year) } else { None },
    month: if form.death_month > 0 { Some(form.death_month) } else { None },
    day: if form.death_day > 0 { Some(form.death_day) } else { None },
  };

  let origin_location = Location {
    country: if form.origin_country.is_empty() { None } else { Some(form.origin_country.clone()) },
    state: if form.origin_state.is_empty() { None } else { Some(form.origin_state.clone()) },
    city: if form.origin_city.is_empty() { None } else { Some(form.origin_city.clone()) },
  };

  let current_location = Location {
    country: if form.current_country.is_empty() { None } else { Some(form.current_country.clone()) },
    state: if form.current_state.is_empty() { None } else { Some(form.current_state.clone()) },
    city: if form.current_city.is_empty() { None } else { Some(form.current_city.clone()) },
  };

  let mut bio = HashMap::new();
  bio.insert("en".into(), "".into());

  let artist = Artist {
    group,
    name,
    birth,
    death,
    origin_location,
    current_location,
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

async fn get_artist(app: web::Data<AppData>, web::Path(artist_id): web::Path<String>) -> HttpResponse {
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
