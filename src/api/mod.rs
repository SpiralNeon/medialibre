use actix_web::web;

pub mod music;

pub fn config(cfg: &mut web::ServiceConfig) {
  cfg
    .service(web::scope("/music").configure(music::config));
}
