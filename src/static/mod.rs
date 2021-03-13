use actix_web::web;

mod music;

pub fn config(cfg: &mut web::ServiceConfig) {
  cfg
    .service(web::scope("/music").configure(music::config));
}
