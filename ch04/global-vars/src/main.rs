#![allow(dead_code, unused)]

use lazy_static::lazy_static;
use once_cell::sync::Lazy;
use static_init::dynamic;
use std::cell::OnceCell;
use std::sync::{Arc, Mutex};

thread_local! {
    static POPULAR_BABY_NAMES_2021: Arc<Mutex<Option<Vec<String>>>> =
        Arc::new(Mutex::new(None));
}

lazy_static! {
  static ref POPULAR_BABY_NAMES_2020: Vec<String> =
    { vec!["Olivia".into(), "Liam".into(), "Emma".into(), "Noah".into(),] };
}

static POPULAR_BABY_NAMES_2019: Lazy<Vec<String>> =
  Lazy::new(|| vec!["Olivia".into(), "Liam".into(), "Emma".into(), "Noah".into()]);

#[dynamic]
static POPULAR_BABY_NAMES_2018: Vec<String> =
  vec!["Emma".into(), "Liam".into(), "Olivia".into(), "Noah".into()];

fn main() {
  let arc = POPULAR_BABY_NAMES_2021.with(|arc| arc.clone());
  let mut inner = arc.lock().expect("unable to lock mutex");
  *inner = Some(vec![
    "Olivia".into(),
    "Liam".into(),
    "Emma".into(),
    "Noah".into(),
  ]);
  let popular_baby_names_2017: OnceCell<Vec<String>> = OnceCell::new();
  popular_baby_names_2017
    .get_or_init(|| vec!["Emma".into(), "Liam".into(), "Olivia".into(), "Noah".into()]);

  println!("popular baby names of 2021: {:?}", *inner);
  println!("popular baby names of 2020: {:?}", *POPULAR_BABY_NAMES_2020);
  println!("popular baby names of 2019: {:?}", *POPULAR_BABY_NAMES_2019);
  println!("popular baby names of 2018: {:?}", *POPULAR_BABY_NAMES_2018);
  println!(
    "popular baby names of 2017: {:?}",
    popular_baby_names_2017.get()
  );
}
