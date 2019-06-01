use std::collections::HashMap;

const RED: &str = "red";
const GREEN: &str = "green";
const YELLOW: &str = "yellow";

// https://www.131002.net/siphash/siphash.pdf

fn main() {
    let mut scores = HashMap::new();

    scores.insert(RED, 10);
    scores.entry(RED).or_insert(100);
    scores.entry(YELLOW).or_insert(100);
    scores.insert(GREEN, 15);
    scores.insert(GREEN, 150);

    println!("scores: {:?}", scores);

    let mut test = HashMap::new();
    test.insert(String::from("TEST"), String::from("hello"));
    println!("{:?}", test);
    println!("{:?}", test.get("TEST"));

    for (key, val) in &scores {
        println!("{}: {}", key, val);
    }

    fn_tuples();
}

fn fn_tuples() {
    let teams = vec![String::from("A"), "b".to_uppercase().to_string()];
    println!("teams: {:?}", teams);
    let initial_scores = vec![5, 10];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    println!("scores: {:?}", scores);
}
