use crate::AppData;
use actix_web::{web, HttpResponse};
use handlebars::Handlebars;

async fn artist(app: web::Data<AppData<'_>>, web::Path(artist_id): web::Path<String>) -> HttpResponse {
  let data = json!({
    "name": artist_id,
  });
  let body = app.hb.render("music/artist", &data).unwrap();

  HttpResponse::Ok().body(body)
}

pub fn config(cfg: &mut web::ServiceConfig) {
  cfg.service(
    web::resource("/artist/{artist_id}")
      .route(web::get().to(artist))
  );
}
