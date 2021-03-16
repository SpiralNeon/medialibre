use serde::{Serialize, Deserialize};

#[derive(PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Language {
  EN,
  ES,
  DE,
  FR,
  JA,
}

use Language::*;

pub fn lang_to_string(lang: Language) -> String {
  match lang {
    EN => "en".into(),
    ES => "es".into(),
    DE => "de".into(),
    FR => "fr".into(),
    JA => "ja".into(),
  }
}

pub fn string_to_lang(s: &str) -> Language {
  match s {
    "en" => EN,
    "es" => ES,
    "de" => DE,
    "fr" => FR,
    "ja" => JA,
    _ => unreachable!(),
  }
}

#[derive(Serialize, Deserialize)]
pub struct Locale {
  short: String,
  long: String,
}

pub fn languages() -> Vec<Locale> {
  vec![
    Locale { short: "en".into(), long: "English".into() },
    Locale { short: "es".into(), long: "Spanish".into() },
    Locale { short: "de".into(), long: "German".into() },
    Locale { short: "fr".into(), long: "French".into() },
    Locale { short: "ja".into(), long: "Japanese".into() },
  ]
}
