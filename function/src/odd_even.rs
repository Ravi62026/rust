use std::io;

fn main() {
    let mut input = String::new();
    println!("Enter a number:");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    // Parse the input to an integer
    let number: i32 = input
        .trim()
        .parse()
        .expect("Please enter a valid number");

    // Call the is_even function with the user's input
    if is_even(number) {
        println!("The number {number} is even");
    }
}

fn is_even(x: i32) -> bool {
    if x % 2 == 0 {
        return true;
    }
    println!("The number {x} is not even");
    false
}
