#![allow(dead_code, unused)]

#[derive(Debug, Clone)]
pub struct Pizza {
  toppings: Vec<String>,
}

impl Pizza {
  pub fn new(toppings: Vec<String>) -> Self {
    Self { toppings }
  }

  pub fn toppings(&self) -> &[String] {
    self.toppings.as_ref()
  }

  pub fn toppings_mut(&mut self) -> &mut Vec<String> {
    &mut self.toppings
  }

  pub fn set_toppings(&mut self, toppings: Vec<String>) {
    self.toppings = toppings;
  }

  pub fn replace_toppings(&mut self, toppings: Vec<String>) -> Vec<String> {
    std::mem::replace(&mut self.toppings, toppings)
  }
}

fn main() {
  let pizza = Pizza::new(vec![
    "tomato sauce".into(),
    "mushrooms".into(),
    "mozzarella".into(),
    "pepperoni".into(),
  ]);
  println!("pizza={:#?}", pizza);
}
