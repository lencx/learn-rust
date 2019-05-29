// use crate::front_of_house::hosting;
use std::collections::HashMap;
use std::fmt;
use std::io::Result as IoResult;

fn main() {
  let mut map = HashMap::new();
  map.insert("a", 2);
  println!("map: {:?}", map);


  let t1 = Triangle { a: 20.0, b: 30.0, c: 50.0 };

  println!("t1: {}", t1);
}

#[derive(Debug)]
struct Triangle {
    a: f32,
    b: f32,
    c: f32,
}

impl fmt::Display for Triangle {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "({}, {}, {})", self.a, self.b, self.c)
  }
}


// fn fn1() -> fmt::Result {}

// fn fn2() -> IoResult<()> {}