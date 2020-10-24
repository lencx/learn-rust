fn main() {
    {
        let r;                                      // -------------+--- 'a
        {                                           //              |
            let x = 5;                              // --+--- 'b    |
            r = &x;                                 //   |          |
            println!("--r: {}", r);                 //   |          |
        }                                           // --+          |
        // `r = &x`                                 //              |
        // borrowed value does not live long enough //              |
        // println!("-r: {}", r);                   // -------------+
    }
    // not found in this scope
    // println!("r: {}", r);

    {
        let x = 5;              // -------------+--- 'b
                                //              |
        let r = &x;             // ---+--- 'a   |
        println!("r: {}", r);   //    |         |
                                // ---+         |
    }                           // -------------+


    let s1 = String::from("abc");
    let s2 = String::from("abcd");

    println!("The longest string is {}", longest(&s1, &s2));

    let result;
    {
        let s3 = String::from("dfdfgra");
        result = longest(&s1, &s3);
    }
    // `s3` dropped here while still borrowed
    // println!("The longest string is {}", result);

    novel();

    println!("first_word: {}", first_word(&"hello world".to_string()));

    let s4: &'static str = "I have a static lifetime.";
    println!("s4: {}", &s4);

    generic_type_parameters_trait_bounds_and_lifetimes_together();
}


fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn novel() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.')
        .next()
        .expect("Could not find a '.'");
    let i = ImportantExcerpt { part: first_sentence };

    println!("novel: {:?}", &i);
    println!("level: {:?}", &i.level());

    println!("----------------");
    println!("novel announce: {}", &i.announce_and_return_part(&String::from("hi, ...")))
}

// fn first_word<'a>(s: &'a str) -> &'a str {
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}


fn generic_type_parameters_trait_bounds_and_lifetimes_together() {
    use std::fmt::Display;

    let a = String::from("aaa");
    let b = String::from("bbbb");

    println!("longest: {}", longest_with_an_announcement(&a, &b, "longest is:"));

    fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
        where T: Display
    {
        println!("Announcement! {}", ann);

        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
}