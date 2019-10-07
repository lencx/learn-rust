use std::ops::Deref;

fn main() {
    let x = 5;

    // let y = &x;
    // assert_eq!(5, y); // error: no implementation for `{integer} == &{integer}`

    // let y = Box::new(x);
    // assert_eq!(5, y); // error: no implementation for `{integer} == std::boxed::Box<{integer}>`

    assert_eq!(5, x);
    // must use the dereference operator to follow the reference to the value it's pointing to.
    // assert_eq!(5, *y);

    let z = MyBox::new(x);
    // assert_eq!(5, z); // error: no implementation for `{integer} == MyBox<{integer}>`
    assert_eq!(5, *z);

    let name = MyBox::new(String::from("Rust"));
    hello(&name);
    // &name[..2]
    hello(&(*name)[..2]); // Hello, Ru
    println!("name: {:?}", name);
}

#[derive(Debug)]
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}", name);
}