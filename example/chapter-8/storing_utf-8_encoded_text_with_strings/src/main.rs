fn main() {
    {
        let mut s = String::new();
        let mut s2 = String::from("你好");
        let mut s3 = "hello world".to_string();
        s.push('A');
        s2.push_str("!!!");
        s3.push_str("!!!");
        println!("s: {}", s);
        println!("s2: {}", s2);
        println!("s3: {}", s3);
        // println!("s2 + s3: {}", s2 + &s3);
        println!("format: {}", format!("{} - {} - {}", s, s2, s3));
    }
    {
        let s = String::from("hello").len();
        println!("{}", s);
    }
    // {
    //     let hello = "Здравствуйте";
    //     let answer = &hello[0..3];
    //     println!("answer: {}", answer);
    // }

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
}
