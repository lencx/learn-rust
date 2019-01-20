fn main() {
    println!("\n============== Scalar Types ============\n");
    int_num();
    float_num();
    numeric_operations();
    bool_type();
    char_type();
    println!("\n============== Compound Types ============\n");
    compound_types_tuple();
    compound_types_array();
}

fn int_num() {
    println!("******** Integer Types ********");
    println!("Decimal => {}", 98_300); // 98300
    println!("Hex => {}", 0xf1); // 241
    println!("Octal => {}", 0o72); // 58
    println!("Binary => {}", 0b1111_0000); // 240
    println!("Byte => {}", b'&'); // 38
}

fn float_num() {
    println!("******** Floating-Point Types ********");
    let x = 2.0;
    let y: f32 = 3.0;
    println!("x => {}, y => {}", x, y);
}

fn numeric_operations() {
    println!("******** Numeric Operations ********");
    // addition
    let sum = 5 + 10;
    // subtraction
    let difference = 34.1 - 4.2;
    // multiplication
    let product = 4 * 9;
    // division
    let quotient = 25.6 / 3.4;
    // remainder
    let remainder = 49 % 3;
    println!("sum => {}; difference => {}; product => {}", sum, difference, product);
    println!("quotient => {}; remainder => {}", quotient, remainder);
}

fn bool_type() {
    println!("******** Boolean Type ********");
    let t = true;
    let f: bool = false;
    println!("t => {}; f => {}", t, f);
}

fn char_type() {
    println!("******** Character Type ********");
    let a = 'A';
    let b = '#';
    let c = '😝';
    println!("a => {}; b => {}; c => {}", a, b, c);
}

fn compound_types_tuple() {
    println!("******** Tuple Type ********");
    let tup: (i32, bool, f32, isize) = (300, true, 3.9, 10);
    let (a, _, _, _) = tup;
    println!("tup => {:#?}", tup);
    println!("a => {}", a);
    println!("tup.2 => {}", tup.2);
}

fn compound_types_array() {
    println!("******** Array Type ********");
    let a = [1, 2, 3, 4, 5];
    let b: [f32; 3] = [1.2, 2.004, 4.3];
    println!("a => {:?}; b => {:?}", a, b);
    println!("a[1] => {}", a[1]);
    /* index out of bounds: the len is 5 but the index is 5 */
    // println!("a[5] => {}", a[5]);
}