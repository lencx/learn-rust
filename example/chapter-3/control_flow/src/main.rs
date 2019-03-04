fn main() {
    let num = 3;
    if num > 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // if 0 {
    //     println!("hello");
    // }

    condition_else_if();
    use_if_in_let();
    loop_again();
    while_loop();
    for_each();
    for_loop();
    for_loop_2();
}

fn condition_else_if() {
    let number = 7; // 8, 9
    // let number = 10;
    if number % 2 == 0 {
        println!("number is divisible by 2");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else {
        println!("number is not divisible by 2 or 3");
    }
}

fn use_if_in_let() {
    let conditon = false;
    let number = if conditon {
        1
    } else {
        2
        // "two"
    };
    println!("The value of number is: {}", number);
}

fn loop_again() {
    let mut count = 0;
    let result = loop {
        println!("again!");
        count += 1;

        if count == 10 {
            break count;
        }
    };

    println!("loop result: {}", result);
    // assert_eq!(result, 11);
}

fn while_loop() {
    let mut num = 5;
    while num != 0 {
        println!("{}!", num);
        num -= 1;
    }
    println!("LIFTOFF!!!");
}


fn for_each() {
    let arr = [1, 4, 6, 8];
    let mut index = 0;

    while index < 4 {
        println!("The value is: {}", arr[index]);
        index += 1;
    }
}

fn for_loop() {
    let arr = ['a', 'c', 'd', 'g'];

    for item in arr.iter() {
        println!("The char is: {}", item);
    }
}

fn for_loop_2() {
    for item in (1..5).rev() {
        println!("{}!", item);
    }
    println!("END!!!");
}