use crate::util::language::Language;
use std::collections::HashMap;
use actix_web::{web, HttpResponse};

struct ArtistName {
  artist_name: String,
  first_name: Option<String>,
  last_name: Option<String>,
  middle_name: Option<String>,
}

struct Date {
  year: Option<u32>,
  month: Option<u32>,
  day: Option<u32>,
}

struct Location {
  country: Option<String>,
  state: Option<String>,
  city: Option<String>,
}

struct Artist {
  group: bool,
  name: HashMap<Language, ArtistName>,
  birth: Date,
  death: Date,
  origin_location: Location,
  current_location: Location,
  members: Vec<String>,
  bio: HashMap<Language, String>,
  images: Vec<String>,
  website: String,
}

pub fn config(cfg: &mut web::ServiceConfig) {
  cfg.service(
    web::resource("/{artist_id}")
      .route(web::get().to(|| HttpResponse::Ok()))
  );
}
