fn main() {
    let v1 = vec![1, 4, 7];
    let v1_iter = v1.iter();
    let mut v1_iter2 = v1.iter();
    println!("v1_iter: {:?}", v1_iter); // Iter([1, 4, 7])

    println!("v1 item: ");
    for (index, item) in v1_iter.enumerate() {
        println!("[{}]: {}", index, item);
    }

    println!("{:?}", v1_iter2.next());
    println!("{:?}", v1_iter2.next());
    println!("{:?}", v1_iter2.next());
    println!("{:?}", v1_iter2.next());

    let v2: Vec<_> = v1.iter().map(|x| x * x).collect();
    println!("v2: {:?}", v2);

    let shoes = vec![
        Shoe { size: 36, style: String::from("sneaker") },
        Shoe { size: 38, style: String::from("sandal") },
        Shoe { size: 36, style: String::from("boot") },
    ];
    let my_shoes = Shoe::shoes_in_my_size(shoes, 36);

    println!("my shoes: {:?}", my_shoes);

    let mut count = Counter::new();

    println!("********** counter next *********");
    println!("next: {:?}", count.next());
    println!("next: {:?}", count.next());
    println!("next: {:?}", count.next());
    println!("next: {:?}", count.next());

    let count_sum: u32 = Counter::new().zip(Counter::new().skip(1))
        .map(|(a, b)| {
            // println!("a: {}, b: {}", a, b);
            a * b
        })
        .filter(|x| x % 3 == 0)
        .sum();
    println!("count_sum: {:?}", count_sum);
}

#[test]
fn v1_sum() {
    let sum: u32 = vec![1, 4, 7].iter().sum();
    assert_eq!(sum, 12);
}

#[derive(Debug)]
struct Shoe {
    size: u32,
    style: String,
}

impl Shoe {
    fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
        shoes.into_iter()
            .filter(|s| s.size == shoe_size)
            .collect()
    }
}

#[derive(Debug)]
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 10 {
            Some(self.count)
        } else {
            None
        }
    }
}