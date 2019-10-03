use std::thread;
use std::time::Duration;

fn main() {
    let simulated_user_specified_value = 23;
    let simulated_random_number = 9;
    println!("Hello, world!");

    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );

    println!("******* closure *******");

    generate_workout_closure(
        simulated_user_specified_value,
        simulated_random_number
    );

    add_one();
}

fn generate_workout(intensity: u32, random_number: u32) {
    if intensity < 25 {
        println!(
            "Today, do {} pushups.",
            simulated_expensive_calculation(intensity)
        );
    } else {
        if random_number == 6 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                simulated_expensive_calculation(intensity)
            );
        }
    }
}

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn generate_workout_closure(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    // let expensive_closure = |num| {
    //     println!("calculating slowly...");
    //     thread::sleep(Duration::from_secs(2));
    //     num
    // };

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            // expensive_closure(intensity)
            expensive_result.value(intensity)
        );
        println!(
            "Next, do {} situps!",
            // expensive_closure(intensity)
            expensive_result.value(intensity)
        );
    } else {
        if random_number == 3 {
            print!("Take a break today! Remember to stay hydrated!");
        } else {
            print!(
                "Today, run for {} minutes!",
                // expensive_closure(intensity)
                expensive_result.value(intensity)
            );
        }
    }
}


fn add_one() {
    fn add_one_v1 (x: u32) -> u32 { x + 1 }
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x| { x + 1 };
    let add_one_v4 = |x| x + 1;

    println!("******* add one **********");

    println!("v2: {}", add_one_v2(10));
    println!("v3: {}", add_one_v3(10));
    println!("v1: {}", add_one_v1(10));
    println!("v4: {}", add_one_v4(10));
}

struct Cacher<T>
    where T: Fn(u32) -> u32
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
    where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, args: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(args);
                self.value = Some(v);
                v
            },
        }
    }
}

#[test]
// FAILED
fn call_with_different_value() {
    let mut c = Cacher::new(|a| a);
    let v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v1, 1);
    assert_eq!(v2, 2);
}

#[test]
// ok
fn captur_env_closure() {
    let x = 4;
    let equal_to_x = |a| a == x;

    let y = 4;

    assert!(equal_to_x(y));
}

/*
#[test]
// can't capture dynamic environment in a fn item
fn captur_env_closure_2() {
    let x = 4;
    fn equal_to_x(a: u32) -> bool { a == x }

    let y = 4;

    assert!(equal_to_x(y));
}

#[test]
// borrow of moved value: `x`
fn move_closures() {
    let x = vec![1, 3, 5];
    let equal_to_x = move |a| a == x;

    println!("can't use x here: {:?}", x);

    let y = vec![1, 3, 5];

    assert!(equal_to_x(y));
}
*/
