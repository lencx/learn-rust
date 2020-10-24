use crate::List::{ Cons, Nil };

fn main() {
    let b = Box::new(5);
    println!("b: {}", b);

    // placing the items next to one another rather than inside one another.
    let list = Cons(1,
                Box::new(Cons(2,
                    Box::new(Cons(3,
                        Box::new(Nil))))));

    println!("List: {:?}", list);
}

/*
            Cons
|--------------------------------|
|   |           Cons             |
|   |----------------------------|
|i32|   |          Cons          |
|   |i32|------------------------|
|   |   |i32|        ∞           |
|--------------------------------|
*/

/*
            Cons
|--------------------------------|
|   |            Box             |
|   |  |---------------------|   |
|i32|  |        usize        |   |
|   |  |---------------------|   |
|--------------------------------|
*/


#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}