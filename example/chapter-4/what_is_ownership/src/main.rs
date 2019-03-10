fn main() {
    {
        let s = "Hello";
        println!("{}", s);
    } // this scope is now over, and s is no longer valid
    // println!("{}", s);
    string_type();
    variable_copy_move();
    variable_clone();
    ownership_and_functions();
    return_values_and_scope();
    return_multiple_values();
}


fn string_type() {
    let mut s = String::from("hello");
    s.push_str(", wrold");
    println!("{}", s);
}

fn variable_copy_move() {
    println!("******* Copy *******");
    let x = 2;
    let y = x;
    println!("x: {}, y: {}", x, y);


    println!("******* Move *******");
    let s1 = String::from("Hi, move");
    let s2 = s1;
    // println!("s1: {}", s1); // value borrowed here after move
    println!("s2: {}", s2);
}

fn variable_clone() {
    println!("******* Clone *******");
    let s1 = String::from("Hi, clone");
    let s2 = s1.clone();
    println!("s1: {}, s2: {}", s1, s2);
}

fn ownership_and_functions() {
    println!("******* Ownership and Functions *******");
    let s = String::from("hello");
    takes_ownership(s);

    let x = 3;
    makes_copy(x);
}

fn takes_ownership(some_string: String) {
    println!("some string: {}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("some integer: {}", some_integer);
}

fn return_values_and_scope() {
    println!("******* Return values and scope *******");
    let s1 = gives_ownership();
    let s2 = String::from("Hello");
    let s3 = takes_and_gives_back(s2);
    println!("s1: {}, s3: {}", s1, s3);
}

fn gives_ownership() -> String {
    let s = String::from("hello");
    s
}

fn takes_and_gives_back(other_string: String) -> String {
    other_string
}

fn return_multiple_values() {
    println!("******* Return multiple values *******");
    let s1 = String::from("hello, world");
    let (s2, len) = calc_len(s1);
    println!("The length of '{}' is {}", s2, len);
    println!("{:?}", calc_len("Hi, len".to_string()));
}

fn calc_len(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}