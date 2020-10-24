// Copyright 2020 lencx
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use std::{thread::sleep, time::Duration};

enum UpdateResult {
  None,
  QuitApplication,
}

struct App {
  client: Box<dyn Client>,
  state: AppState,
}

struct AppState {
  title: String,
}

trait Client {
  // returns false if the app should exit
  fn update(&mut self) -> UpdateResult;
  fn render(&self);
}

struct MyClient {
  ticks_left: usize,
}

impl Client for MyClient {
  fn update(&mut self) -> UpdateResult {
    self.ticks_left -= 1;
    if self.ticks_left > 0 {
      UpdateResult::None
    } else {
      UpdateResult::QuitApplication
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
    println!("=== Your are now playing {} === ", self.state.title);

    loop {
      let result = self.client.update();
      self.client.render();

      match result {
        UpdateResult::None => {},
        UpdateResult::QuitApplication => break,
      }

      sleep(Duration::from_secs(1))
    }
  }
}

fn main() {
  let client = MyClient { ticks_left: 4 };

  let mut app = App {
    state: AppState {
      title: String::from("Jack in the box"),
    },
    client: Box::new(client),
  };

  app.run();
}
