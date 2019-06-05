fn main() {
    let number_list = vec![10, 20, 40, 60];
    let mut largest = number_list[0];
    for number in number_list {
        if number > largest {
            largest = number;
        }
    }
    println!("The largest number is {}", largest);

    let number_list2 = vec![49, 20, 10, 69, 43, 82, 54];
    let largest2 = max(&number_list2);
    println!("The largest number is {}", largest2);
}


fn max(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item
        }
    }

    largest
}