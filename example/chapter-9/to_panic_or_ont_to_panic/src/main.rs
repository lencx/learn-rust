use std::{ io, net::IpAddr, cmp::Ordering };
use rand::Rng;

fn main() {
    get_home_ip();
    guessing_game();
}

#[derive(Debug)]
pub struct Guess {
    value: i32,
}

impl Guess {
	pub fn new(value: i32) -> Guess {
		if value < 1 || value > 100 {
			panic!("Guess value must be between 1 and 100, got {}.", value);
		}

		Guess {
			value
		}
	}

	pub fn value(&self) -> i32 {
		self.value
	}
}

fn guessing_game() {

    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        // .expect("Please type a number!");
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("ERROR:=> Please type a number!");
                continue;
            },
        };

        Guess::new(guess);
        // if guess < 1 || guess > 100 {
        //     println!("The secret number will be between 1 and 100.");
        //     continue;
        // }

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

fn get_home_ip() {
    let home: IpAddr = "127.0.0.1".parse().unwrap();
    println!("home: {}", home);
    println!("------------------------");
}
