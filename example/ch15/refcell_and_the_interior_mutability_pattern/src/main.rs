use std::rc::Rc;
use std::cell::RefCell;
use crate::List::{Cons, Nil};

fn main() {
    let _x = 5;
    // let y = &mut x; // cannot borrow as mutable

    let val = Rc::new(RefCell::new(1));
    let a = Rc::new(Cons(Rc::clone(&val), Rc::new(Nil)));
    let b = Cons(Rc::new(RefCell::new(2)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));

    println!("a: {:?}", a);
    println!("b: {:?}", b);
    println!("c: {:?}", c);

    *val.borrow_mut() += 10;
    println!("a+10: {:?}", a); // 11
    *val.borrow_mut() += 10;
    println!("b+10: {:?}", b); // 21
    *val.borrow_mut() += 10;
    println!("c+10: {:?}", c); // 31
}


#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}
