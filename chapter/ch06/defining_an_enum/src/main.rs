fn main() {
    test_ip();
    test_ip2();
    enum_example();
    option_example();
}

fn test_ip() {
    #[derive(Debug)]
    enum IpAddrKind {
        V4,
        V6,
    }
    #[derive(Debug)]
    struct IpAddr<'a> {
        kind: IpAddrKind,
        address: &'a str,
    }
    let four = IpAddrKind::V4;
    println!("four: {:?}", four);


    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: "127.0.0.1",
    };
    let school = IpAddr {
        kind: IpAddrKind::V6,
        address: "::1",
    };

    println!("home: {:?}, \nschool: {:?}", home, school);
}

fn test_ip2() {
    println!("------------------------");

    #[derive(Debug)]
    enum IpAddr<'a> {
        V4(&'a str),
        V6(String),
        V4_1(u8, u8, u8, u8),
    }

    let home = IpAddr::V4("127.0.0.1");
    let home2 = IpAddr::V4_1(127, 0, 0, 1);
    let school = IpAddr::V6(String::from("::1"));
    println!("home: {:?}; \nhome2: {:?}; \nschool: {:?}", home, home2, school);
}

fn enum_example() {
    println!("------------------------");

    // struct QuitMessage;
    // struct MoveMessage { x: i32, y: i32 };
    // struct WriteMessage(String);
    // struct ChangeColorMessage(i32, i32, i32);
    #[derive(Debug)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    impl Message {
        fn call(&self) {
            println!("{:?}", self);
        }
    }

    let msg = Message::Write(String::from("hello world"));
    msg.call();
}

fn option_example() {
    println!("------------------------");

    #[derive(Debug)]
    enum Option<T> {
        Some(T),
        None,
    }
    let some_number = Some(6);
    let some_string = Some(String::from("a string"));
    // let absent_number: Option<i32> = None;

    println!("{:?}; {:?}", some_number, some_string);
}