#![allow(dead_code, unused)]

use std::any::Any;

trait MyTrait {
  fn hello(&self);
  fn as_any(&self) -> &dyn Any;
}

struct MyStruct1;

impl MyStruct1 {
  fn struct_hello(&self) {
    println!("Hello, world! from MyStruct1");
  }
}

impl MyTrait for MyStruct1 {
  fn hello(&self) {
    self.struct_hello();
  }
  fn as_any(&self) -> &dyn Any {
    self
  }
}

struct MyStruct2;

impl MyStruct2 {
  fn struct_hello(&self) {
    println!("Hello, world! from MyStruct2");
  }
}

impl MyTrait for MyStruct2 {
  fn hello(&self) {
    self.struct_hello();
  }
  fn as_any(&self) -> &dyn Any {
    self
  }
}

fn main() {
  let mut v: Vec<Box<dyn MyTrait>> = vec![Box::new(MyStruct1), Box::new(MyStruct2)];

  v.iter().for_each(|i| i.hello());

  // no method named `struct_hello` found for reference `&std::boxed::Box<dyn MyTrait>`
  // v.iter().for_each(|i| i.struct_hello());

  println!("With a downcast:");
  v.iter().for_each(|i| {
    if let Some(obj) = i.as_any().downcast_ref::<MyStruct1>() {
      obj.struct_hello();
    }

    if let Some(obj) = i.as_any().downcast_ref::<MyStruct2>() {
      obj.struct_hello();
    }
  });
}
