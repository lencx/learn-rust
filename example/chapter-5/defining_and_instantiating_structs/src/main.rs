fn main() {
    let mut mine = User {
        username: "Len",
        age: 26,
        active: true,
    };

    mine.active = false;

    println!("mine: {:#?}", mine);

    let test = build_user("Test", 30);
    println!("test user: {:?}", test);

    let cp_mine = User {
        age: 29,
        ..mine
    };
    println!("cp_mine: {:?}", cp_mine);

    println!("******** Tuples ********");
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("black: {:?}; \norigin: {:?};", black, origin);
}

#[derive(Debug)]
struct User<'a> {
    username: &'a str,
    age: u8,
    active: bool,
}


fn build_user(username: &str, age: u8) -> User {
    User {
        username,
        age,
        active: true,
    }
}

#[derive(Debug)]
struct Color(i32, i32, i32);

#[derive(Debug)]
struct Point (i32, i32, i32);