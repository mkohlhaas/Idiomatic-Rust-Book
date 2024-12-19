#![allow(dead_code, unused)]

struct Dog;
struct Cat;

pub trait SelfDescribing {
  fn describe() -> String;
}

impl SelfDescribing for Dog {
  fn describe() -> String {
    "happy little dog".into()
  }
}

impl SelfDescribing for Cat {
  fn describe() -> String {
    "curious cat".into()
  }
}

fn describe_type<T: SelfDescribing>() -> String {
  T::describe()
}

fn main() {
  println!("I am a {}", describe_type::<Dog>());
  println!("I am a {}", describe_type::<Cat>());
}
