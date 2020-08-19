// Copyright 2020 lencx
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.
// @see: https://fasterthanli.me/articles/i-am-a-java-csharp-c-or-cplusplus-dev-time-to-do-some-rust
// @cmd: cargo run --quiet

use std::{thread::sleep, time::Duration};


// no lifetime parameter
struct App {
  title: String,
  running: bool,
  // invalid `self` parameter type: &mut dyn Client
  // client: &'a mut dyn Client
  // raw pointer
  client: *mut dyn Client,
}

trait Client {
  fn update(&mut self, app: *mut App);
  fn render(&self);
}

struct MyClient {
  ticks_left: usize,
}

impl Client for MyClient {
  fn update(&mut self, app: *mut App) {
    self.ticks_left -= 1;
    if self.ticks_left == 0 {
      // this is fine, probably
      unsafe {
        (*app).running = false;
      }
    }
  }
  fn render(&self) {
    if self.ticks_left > 0 {
      println!("You turn the crank...");
    } else {
      println!("Jack POPS OUT OF THE BOX");
    }
  }
}

impl App {
  fn run(&mut self) {
    println!("=== Your are now playing {} ===", self.title);
    loop {
      // this converts a reference to a raw pointers
      let app = self as *mut _;
      // this converts a raw pointer to a reference
      let client = unsafe { self.client.as_mut().unwrap() };
      // ..which we need because the receiver is a reference
      client.update(app);
      client.render();

      if !self.running {
        break;
      }

      sleep(Duration::from_secs(1));
    }
  }
}

fn main() {
  let mut client = MyClient {
    ticks_left: 4,
  };

  let mut app = App {
    title: String::from("Jack in the box"),
    running: true,
    client: &mut client,
  };

  app.run();
}
