use std::io;
use std::cmp::Ordering;
// add `rand` crate to Cargo.toml
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        // .expect("Please type a number!");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("ERROR:=> Please type a number!");
                continue;
            },
        };

        println!("You guessed: {}", guess);

        // comparing two numbers
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("    Result:=> Too small!"),
            Ordering::Greater => println!("    Result:=> Too big!"),
            Ordering::Equal => {
                println!("   Result:=> You win!");
                break;
            },
        }
    }
}
