fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(5);
    println!("v: {:?}", v);
    {
        let mut v2 = vec![1, 3, 5];
        v2.push(7);
        println!("v2: {:?}", v2);
        println!("The second element is {:?}", &v2[1]);

        match v2.get(2) {
            Some(val) => println!("The third element is {:?}", val),
            None => println!("There is no third element"),
        }
    } // <- v2 goes out of scope and is freed here
    // println!("v2: {:?}", v2); // cannot find value `v2` in this scope

    {
        let mut v = vec![2, 4, 6, 8];
        println!("{:?}, {:?}", &v[3], v.get(3));
        // println!("does not exist: {:?}", &v[100]);
        println!("does not exist: {:?}", v.get(100));
        // let first = &v[0];
        v.push(10);
        // println!("The first element is {:?}", first);

        for i in &v {
            println!("{}", i);
        }

        for i in &mut v {
            *i += 50;
            println!("{} ", i)
        }
    }
    multiple_type();
}

fn multiple_type() {
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(8),
        SpreadsheetCell::Float(8.9),
        SpreadsheetCell::Text(String::from("Hello")),
    ];

    println!("row: {:#?}", row)
}