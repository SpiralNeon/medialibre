use actix_web::web;

pub mod artist;
pub mod genre;
pub mod label;
pub mod release;

pub fn config(cfg: &mut web::ServiceConfig) {
  cfg
    .service(web::scope("/artist").configure(artist::config))
  	.service(web::scope("/genre").configure(genre::config))
  	.service(web::scope("/label").configure(label::config))
  	.service(web::scope("/release").configure(release::config));
}
