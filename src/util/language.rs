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
    EN => "en".to_string(),
    ES => "es".to_string(),
    DE => "de".to_string(),
    FR => "fr".to_string(),
    JA => "ja".to_string(),
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
