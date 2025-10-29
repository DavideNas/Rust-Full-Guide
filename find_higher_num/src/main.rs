fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn is_even(num: i32) -> bool {
    let element = num % 2;
    element == 0
}

fn main() {
    let numbers = vec![34, 50, 25, 99, 65];
    let result = largest(&numbers);
    println!(
        "The largest number is {}, wich is even = {}",
        result,
        is_even(result)
    );
}
