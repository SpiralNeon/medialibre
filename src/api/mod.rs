use actix_web::{web, HttpResponse};
use actix_session::Session;

pub mod music;

async fn select_lang(web::Path((lang, location)): web::Path<(String, String)>, session: Session) -> HttpResponse {
  session.set("lang", &lang).unwrap();

  HttpResponse::SeeOther()
    .header("Location", location.replace("\\", "/"))
    .body(lang)
}

pub fn config(cfg: &mut web::ServiceConfig) {
  cfg
    .route("/select-lang?lang={lang}&location={location}", web::get().to(select_lang))
    .service(web::scope("/music").configure(music::config));
}
