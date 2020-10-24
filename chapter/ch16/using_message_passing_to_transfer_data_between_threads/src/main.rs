use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();
    let _tx = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        let msg = String::from("Hello, Rust!");
        _tx.send(msg).unwrap();
        // println!("msg is {}", msg); // value borrowed here after move

        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for item in vals {
            _tx.send(item).unwrap();
            thread::sleep(Duration::from_secs(2));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for item in vals {
            tx.send(item).unwrap();
            thread::sleep(Duration::from_secs(2));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }

    // let received = rx.recv().unwrap();
    // println!("Got: {}", received);
}
