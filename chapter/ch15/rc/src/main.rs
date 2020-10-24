use std::rc::Rc;

fn main() {
    // box_cons();
    rc_cons();
    rc_count();
}

// fn box_cons() {
//     use crate::BoxList::{ Cons, Nil };
//     let a = Cons(1,
//                 Box::new(Cons(2,
//                     Box::new(Cons(3,
//                         Box::new(Nil))))));
//     let _b = Cons(4, Box::new(a));  // a: value moved here
//     let _c = Cons(5, Box::new(a));  // a: value used here after move
// }

// #[derive(Debug)]
// enum BoxList {
//     Cons(i32, Box<BoxList>),
//     Nil,
// }

// the call to `Rc::clone` only increments the reference count,
// ehich  doesn't take much time.
fn rc_cons() {
    use crate::RcList::{ Cons, Nil };
    let a = Rc::new(Cons(1,
                Rc::new(Cons(2,
                    Rc::new(Cons(3,
                        Rc::new(Nil)))))));
    let b = Cons(4, Rc::clone(&a));
    let c = Cons(5, Rc::clone(&a));

    println!("rc-b: {:?}", b);
    println!("rc-c: {:?}", c);
}

// `Rc<T>` allows you to share data between multiple parts of your program for reading only.
#[derive(Debug)]
enum RcList {
    Cons(i32, Rc<RcList>),
    Nil,
}

fn rc_count() {
    use crate::RcList::{ Cons, Nil };

    println!("******** Rc Count ********");
    let a = Rc::new(Cons(1, Rc::new(Nil)));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let _b = Cons(2, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let _c = Cons(3, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}