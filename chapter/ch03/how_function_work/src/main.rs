fn main() {
    println!("Hello, world!");
    another_func();
    func_param(8);
    func_param_type(9, 7.8);
    func_contain_statements_expressions();
    func_return();
}

fn another_func() {
    println!("Another function!");
}

fn func_param(x: i32) {
    println!("The value of x is: {}", x);
}

fn func_param_type(x: i32, y: f32) {
    println!("x => {}; y => {}", x, y);
}

fn func_contain_statements_expressions() {
    let x = 5;
    let y = {
        let x = x + 2;
        x + 1
    };
    println!("x => {}; y => {}", x, y);
}

fn func_return() {
    println!("sum => {}", sum(10, 15));
}

fn sum(x: i32, y: i32) -> i32 {
    x + y
}