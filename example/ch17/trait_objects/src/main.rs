use gui::{Draw, Screen, Button};

#[derive(Debug)]
struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // code to actuallly draw a select box
    }
}

fn main() {
    println!("Hello, world!");
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 80,
                height: 20,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 20,
                label: String::from("Ok"),
            }),
            // the trait `gui::Draw` is not implemented for `std::string::String`
            // Box::new(String::from("Hello")),
        ]
    };

    screen.run();
}

// pub trait Clone {
//     fn clone(&self) -> Self;
// }

// pub struct Screen2 {
//     //  the trait `Clone` cannot be made into an object
//     pub components: Vec<Box<dyn Clone>>,
// }