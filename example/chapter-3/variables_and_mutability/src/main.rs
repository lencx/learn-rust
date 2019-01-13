fn main() {
    // constant_a();
    mut_b();
    constant();
    shadows();
    str_len();
}

// fn constant_a() {
//     let a = 3;
//     println!("The value of a is: {}", a);
//     a = 6;
//     println!("The value of a is: {}", a)
// }

fn mut_b() {
    let mut b = 6;
    println!("The value of b is: {}", b);
    b = 9;
    println!("The value of b is: {}", b);
}

fn constant() {
    const MAX_POINTS: u32 = 100_000_000;
    println!("MAX POINTS :=> {}", MAX_POINTS);
}

fn shadows() {
    let _x = 10;
    let _x = 10 * 10;
    let _x = _x / 2;
    println!("The value of _x is: {}", _x);
}

fn str_len() {
    let str = "hello, world!!";
    let str = str.len();
    println!("str_len :=> {}", str);
}