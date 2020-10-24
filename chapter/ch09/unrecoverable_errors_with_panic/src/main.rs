fn main() {
    println!("Hello, world!");
    // panic!("crash and burn");

    let v = vec![1, 2, 4];
    // RUST_BACKTRACE=1 cargo run --release
    v[5];
}
