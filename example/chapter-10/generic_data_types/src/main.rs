fn main() {
    let num_list = vec![1, 5, 10, 8, 4];
    let max_num = largest_i32(&num_list);
    println!("The largest number is: {}", max_num);
    let char_list = vec!['d', 'z', 'm', 'D', 'S'];
    let max_char = largest_char(&char_list);
    println!("The largest char is: {}", max_char);
    // let max_n = largest(&num_list);
    // let max_c = largest(&char_list);
    // println!("number: {}, char: {}", max_n, max_c);

    println!("---struct definitions---");
    let intPoint = Point { x: 10, y: 20 };
    let floatPoint = Point { x: 10.5, y: 20.3 };
    let mixPoint = Point2 { x: 10, y: 20 };
    let mixPoint2 = Point2 { x: 10.3, y: 20.9 };
    let p3 = mixPoint.mixup(mixPoint2);
    println!("intPoint: {:?}, floatPoint: {:?}", intPoint, floatPoint);
    println!("distance_from_origin:{:?}", floatPoint.distance_from_origin());
    println!("mix: p3.x = {}; p3.y = {}", p3.x, p3.y);
}


fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item < largest {
            largest = item
        }
    }

    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list.iter() {
        if item < largest {
            largest = item
        }
    }

    largest
}

// fn largest<T>(list: &[T]) -> T {
//     let mut largest = list[0];

//     for &item in list.iter() {
//         if item < largest {
//             largest = item
//         }
//     }

//     largest
// }

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

#[derive(Debug)]
struct Point2<T, U> {
    x: T,
    y: U,
}
impl<T, U> Point2<T, U> {
    fn mixup<V, W>(self, other: Point2<V, W>) -> Point2<T, W> {
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}