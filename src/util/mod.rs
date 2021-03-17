use crate::Language;

pub fn languages() -> Vec<Language> {
  vec![
    Language { short: "en".into(), long: "English".into() },
    Language { short: "es".into(), long: "Español".into() },
  ]
}
