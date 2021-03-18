use crate::AppData;
use std::fs;
use actix_web::web;
use redis::Commands;

fn music_new_artist(app: &web::Data<AppData>) {
  let new_data = fs::read_to_string("static/templates/music/new/artist.html").unwrap();
  let old_data: Option<String> = app.rdb.lock().unwrap().hget("templates", "music/new/artist").unwrap();

  let update = || {
    let rdb = &mut app.rdb.lock().unwrap();
    let _: () = rdb.del("music-new-artist").unwrap();
    let _: () = rdb.hset("templates", "music/new/artist", &new_data).unwrap();
  };

  match old_data {
  	Some(old_data) => {
  	  if new_data != old_data {
        update();
  	  }
  	},
  	None => update(),
  }
}

fn music_artist(app: &web::Data<AppData>) {
  let new_data = fs::read_to_string("static/templates/music/artist.html").unwrap();
  let old_data: Option<String> = app.rdb.lock().unwrap().hget("templates", "music/artist").unwrap();

  let update = || {
    let rdb = &mut app.rdb.lock().unwrap();
    let ids: Vec<String> = rdb.smembers("music-artists").unwrap();
    for id in ids {
      let _: () = rdb.del(format!("music-artist-{}", id)).unwrap();
    }
    let _: () = rdb.del("music-artists").unwrap();
    let _: () = rdb.hset("templates", "music/artist", &new_data).unwrap();
  };

  match old_data {
    Some(old_data) => {
      if new_data != old_data {
        update();
      }
    },
    None => update(),
  }
}

pub fn clean_cache(app: web::Data<AppData>) {
  music_new_artist(&app);
  music_artist(&app);
}
