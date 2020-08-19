// Copyright 2020 lencx
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.
// @see: https://fasterthanli.me/articles/i-am-a-java-csharp-c-or-cplusplus-dev-time-to-do-some-rust
// @cmd: cargo run --quiet

use std::{thread::sleep, time::Duration};

struct App {
  title: String,
  running: bool,
  ticks_left: usize,
}

impl App {
  fn update(&mut self) {
    self.ticks_left -= 1;
    if self.ticks_left == 0 {
      self.running = false;
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

fn main() {
  let mut app = App {
    title: String::from("Jack in the box"),
    ticks_left: 4,
    running: true,
  };

  println!("=== Your are now playing {} ===", app.title);

  loop {
    app.update();
    app.render();

    if !app.running {
      break;
    }

    sleep(Duration::from_secs(1));
  }
}

// // globals are bad, and I shouldn't use them.
// // const WIDTH: usize = 1280;
// // const HEIGHT: usize = 720;

// // I'm making an app - I can make an App `struct`
// struct App {
//     dimensions: Dimensions,
//     title: String,
// }

// struct Dimensions {
//     width: usize,
//     height: usize,
// }

// fn main() {
//     let app = App {
//         dimensions: Dimensions {
//             width: 1280,
//             height: 720,
//         },
//         title: "My App".to_string(),
//     };
//     // println!("Should make a {}x{} window.", WIDTH, HEIGHT);
//     println!(
//         "Should make a {}x{} window with title {}",
//         app.dimensions.width, app.dimensions.height, app.title
//     );
// }
