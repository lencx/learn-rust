fn main() {
    // [72, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100, 33]
    let mut s = String::from("Hello world!");
    let s2 = "Hello world!";
    let word = first_word(&s);
    let word2 = first_word2(&s2);
    println!("first word: {}", word);
    // s.clear(); // ""
    println!("first word2: {}", word2); // immutable borrow later used here
    s.clear(); // ""
    println!("s: {}, word: {}", s, word); // s: , word: 6
    str_slices();
    str_cn();
    other_slice();
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    println!("bytes: {:?}", bytes);
    println!("b' ': {:?}", b' '); // 32
    for (i, &item) in bytes.iter().enumerate() {
        println!("i: {}, &item: {}", i, item); // i => index, &item => byte element
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn str_slices() {
    println!("******** String slices ********");
    let s = String::from("hello, world");
    let hello = &s[..5];
    let world = &s[6..]; // [6..=11] || [6..len]

    println!("hello: {}    world: {}", hello, world);
}

fn str_cn() {
    let s = String::from("你好，世界");
    for c in s.chars() {
        print!("[{}]", c);
    }
    println!("");
}

fn first_word2(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    return &s[..];
}

fn other_slice() {
    println!("******** Other slices ********");
    let a = [1, 2, 3, 4, 5, 6];
    let b = &a[1..4];
    println!("b: {:?}", b);
}