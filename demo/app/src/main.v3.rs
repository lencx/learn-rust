// Copyright 2020 lencx
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.
// @see: https://fasterthanli.me/articles/i-am-a-java-csharp-c-or-cplusplus-dev-time-to-do-some-rust
// @cmd: cargo run --quiet

use std::{
  fmt::Display,
  sync::Arc,
  thread::sleep,
  time::{SystemTime, Duration},
};

static FOREVER_CLIENT: MyClient = MyClient { ticks_left: 1 };
static mut GLOBAL_LOGGER: Option<&'static Logger> = None;

struct MyClient {
  ticks_left: usize,
}

struct Logger {}

fn show_number(x: i32) {
  println!("x = {}", x)
}

// fn show_generic<T: Display>(x: T) {
//   println!("x = {}", x)
// }
fn show_generic<T>(x: T)
  where T: Display {
  println!("x = {}", x)
}

fn show_ticks(mc: &'static MyClient) {
  println!("{} ticks left", mc.ticks_left);
}

fn set_logger(logger: &'static Logger) {
  unsafe {
    GLOBAL_LOGGER = Some(logger);
  }
}

fn logger_msg(timestamp: SystemTime, message: &str) {
  println!("[{:?}] {}", timestamp, message);
}

// ========================== Journal =====================================

//=== v1
// #[derive(Default, Debug)]
// struct Journal<'a> {
//   messages: Vec<&'a str>,
// }

// impl<'a> Journal<'a> {
//   fn new() -> Self {
//     Journal { messages: Vec::new() }
//   }
//   fn log(&mut self, msg: &'a str) {
//     self.messages.push(msg);
//   }
// }

// fn get_journal<'a>() -> Journal<'a> {
//   // let mut journal: Journal = Default::default();
//   let mut journal = Journal::new();
//   journal.log("This a bright morning");
//   journal.log("This wind is howling");
//   journal
// }

//=== v2
// #[derive(Default, Debug)]
// struct Journal {
//   messages: Vec<Arc<String>>,
// }

// impl Journal {
//   fn log(&mut self, msg: Arc<String>) {
//     self.messages.push(msg);
//   }
// }

// fn get_journal() -> Journal {
//   let mut journal: Journal = Default::default();
//   journal.log(Arc::new(String::from("This a bright morning")));
//   journal.log(Arc::new(String::from("This wind is howling")));
//   journal
// }

//=== v3
#[derive(Default, Debug)]
struct Journal {
  events: Vec<Arc<Event>>,
}

#[derive(Debug)]
struct Event {
  message: String,
}

impl Event {
  fn print(&self) {
    println!("Event(messgae={})", self.message);
  }
}

impl Journal {
  fn log(&mut self, message: String) {
    self.events.push(Arc::new(Event { message }));
  }
  fn last_event(&self) -> Option<Arc<Event>> {
  // fn last_event(&self) -> Option<&Event> {
    // self.events.last().map(|e| e.as_ref())
    // self.events.last().map(|e| Arc::clone(e))
    // shorthand
    self.events.last().cloned()
  }
}

fn get_journal() -> Journal {
  let mut journal: Journal = Default::default();
  journal.log(String::from("event: click"));
  journal.log(String::from("event: scroll"));
  journal.log(String::from("event: input"));
  journal
}

fn print_event() {
  let ev = Arc::new(Event { message: String::from("click") });
  ev.print();
}

// ===============================================================

fn main() {
  show_number(5);
  show_generic("lencx");
  show_generic(4);
  show_generic(true);

  show_ticks(&FOREVER_CLIENT);

  let client = MyClient { ticks_left: 1 };
  show_ticks(Box::leak(Box::new(client)));

  let logger = Logger {};
  set_logger(Box::leak(Box::new(logger)));

  logger_msg(SystemTime::now(), "starting up....");
  sleep(Duration::from_secs(1));
  logger_msg(SystemTime::now(), "shutting down....");

  // let get_journal_msgs: Journal<'static> = get_journal();
  let get_journal_msgs: Journal = get_journal();
  println!("journal: {:?}", get_journal_msgs);
  println!("journal last event: {:?}", get_journal_msgs.last_event());

  print_event();
}

// #[derive(Debug)]
// struct Foobar {}

// fn main() {
//   let f = Foobar {};
//   let f_ref = &f;
//   let f_box = Box::new(Foobar {});

//   println!("f: {:?}", f);
//   println!("f_ref: {:?}", f_ref);
//   println!("f_box: {:?}", f_box);
//   println!("size of &T     = {:?}", std::mem::size_of_val(&f_ref));
//   println!("size of Box<T> = {:?}", std::mem::size_of_val(&f_box));
// }
