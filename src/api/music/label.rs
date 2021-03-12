use actix_web::{web, HttpResponse};

pub fn config(cfg: &mut web::ServiceConfig) {
  cfg.service(
    web::resource("/{label_id}")
      .route(web::get().to(|| HttpResponse::Ok()))
  );
}
