use crate::Language;

pub fn languages() -> Vec<Language> {
  vec![
    Language { short: "en".into(), long: "English".into() },
    Language { short: "es".into(), long: "Español".into() },
  ]
}

pub fn month_days() -> Vec<u32> {
  vec![
  	31,
  	29,
  	31,
  	30,
  	31,
  	30,
  	31,
  	31,
  	30,
  	31,
  	30,
  	31,
  ]
}
