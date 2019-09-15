pub mod utils;

fn main() {
    println!("Hello, world!");
    let num_list = [100, 30, 40,  170, 20, 80];
    println!("The largest number is: {}", utils::largest(&num_list));

    let char_list = ['a', 'c', 'z', 'm', 'q'];
    println!("The largest char is: {}", utils::largest(&char_list));
}


// use std::fmt::Display;

// #[derive(Debug)]
// struct Pair<T> {
//     x: T,
//     y: T,
// }

// impl<T> Pair<T> {
//     fn new(x: T, y: T) -> Self {
//         Self {
//             x,
//             y,
//         }
//     }
// }

// impl<T: Display + PartialOrd> Pair<T> {
//     fn cmp_display(&self) {
//         if self.x >= self.y {
//             println!("the largest number is x: {}", self.x);
//         } else {
//             println!("the largest number is y: {}", self.y);
//         }
//     }
// }