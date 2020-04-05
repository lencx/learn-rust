use std::io::{self, Write};

fn main() {
    // get the global stdout entity
    let stdout = io::stdout();

    // acquire a lock on it
    let mut handle = stdout.lock();
    writeln!(handle, "foo: {}", 42);

    let pb = indicatif::ProgressBar::new(100);
    for i in 0..100 {
        pb.println(format!("[+] finished #{}", i));
        pb.inc(1);
    }
    pb.finish_with_message("done");
}
