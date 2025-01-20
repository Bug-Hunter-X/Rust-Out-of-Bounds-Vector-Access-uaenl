fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    match numbers.get(10) {
        Some(num) => println!("The number is: {}", num),
        None => println!("Index out of bounds"),
    };
} 