fn main() {
    println!("hello");  // 1
    let a = CustomSmartPointer { data: String::from("my stuff") }; // 6
    let b = CustomSmartPointer { data: String::from("other stuff") };  // 5
    let c = CustomSmartPointer { data: String::from("some data") }; // 3
    println!("CustomSmartPointers created.\n    (a: {:?})\n    (b: {:?})", a, b);   // 2
    // c.drop();   // explicit destructor calls not allowed
    drop(c);
    println!("CustomSmartPointer dropped before the end of main."); // 4
    // println!("c: {:?}", c); // value borrowed here after move
}


#[derive(Debug)]
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustSmartPointer with data: {}", self.data);
    }
}