fn main() {
    let width = 40;
    let height = 20;

    println!("The area of the rectangle is {} square pixels.", area(width, height));

    let rect1 = (30, 40);
    println!("rect1 area: {}", area2(rect1));

    let rect2 = Rectangle {
        width: 50,
        height: 40,
    };
    println!("rect2 area: {}", area3(&rect2));
    println!("rect2: {:#?}", rect2);
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}