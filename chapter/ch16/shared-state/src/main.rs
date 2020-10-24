use std::sync::{Mutex, Arc};
// use std::rc::Rc;
use std::thread;

fn main() {
    let x = Mutex::new(5);

    {
        let mut a = x.lock().unwrap();
        *a = 8;
    }
    println!("x: {:?}", x);

    multiple_threads();
}

fn multiple_threads() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    // let a1 = Arc::clone(&counter);
    // let handle01 = thread::spawn(move || {
    //     let mut num = a1.lock().unwrap();
    //     *num += 1;
    // });
    // handle01.join().unwrap();

    // let a2 = Arc::clone(&counter);
    // let handle02 = thread::spawn(move || {
    //     let mut num = a2.lock().unwrap();
    //     *num += 1;
    // });
    // handle02.join().unwrap();

    for _ in 0..10 {
        let a = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = a.lock().unwrap();

            *num += 1;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("counter: {:?}", counter);
    println!("counter: {:?}", *counter.lock().unwrap());
}