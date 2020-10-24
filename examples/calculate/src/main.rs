macro_rules! calculate {
    (eval $e: expr) => {{
        {
            let val: usize = $e; // force types to be integer
            println!("{} = {}", stringify!($e), val);
        }
    }}
}

fn main() {
    calculate! {
        eval 1 + 2
    }
    calculate! {
        eval (2 * 5) + (4 / 6)
    }
}
