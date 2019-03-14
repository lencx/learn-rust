fn main() {
    calculate_length();

    println!("******** Change ********");
    let mut s = String::from("Hello");
    // println!("{:?}", &mut s.push_str("1111"));
    let s2 = change_str(&mut s);
    println!("{:?}", s2);
    // let s3 = String::from("hello");
    // {
    //     let r2 = &mut s3;
    //     println!("r2: {}", r2);
    // }
    // let r1 = &mut s3;
    // println!("r1: {}", r1);
    // dangle();
    println!("{}", dangle2());
}

fn calculate_length() {
    println!("******** Calcculate length ********");
    let s = String::from("Hello, world!");
    println!("s: {}", s);
    println!("{:?}", calc_len(&s));
}

fn calc_len(s: &String) -> (String, usize) {
    (s.to_string(), s.len())
}

fn change_str(s: &mut String) {
    s.push_str(", world")
}

// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// } // s goes out of scope, and is dropped. Its memory goes away.
     // Danger!

fn dangle2() -> String {
    let s = String::from("hello");
    s
}