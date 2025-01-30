// Calculator

use std::io;

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn sub(a: i32, b: i32) -> i32 {
    a - b
}

fn mul(a: i32, b: i32) -> i32 {
    a * b
}

fn div(a: i32, b: i32) -> Option<f64> {
    if b == 0 {
        return None;
    }
    Some((a as f64) / (b as f64))
}

fn square_root(a: i32) -> f64 {
    (a as f64).sqrt()
}

fn get_number(prompt: &str) -> i32 {
    loop {
        let mut input = String::new();
        println!("{}", prompt);

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        // ::<T>  -> turbo fish symbol
        match input.trim().parse::<i32>() {
            Ok(num) => return num, // Return parsed number
            Err(_) => {
                println!("Invalid input! Please enter a valid number.");
                continue;
            }
        }
    }
}

fn main() {
    let input_one = get_number("Enter the first number:");
    let input_two = get_number("Enter the second number:");

    println!("\nResults:");
    println!("Addition: {} + {} = {}", input_one, input_two, add(input_one, input_two));
    println!("Subtraction: {} - {} = {}", input_one, input_two, sub(input_one, input_two));
    println!("Multiplication: {} * {} = {}", input_one, input_two, mul(input_one, input_two));

    match div(input_one, input_two) {
        Some(result) => println!("Division: {} / {} = {}", input_one, input_two, result),
        None => println!("Division: Cannot divide by zero!"),
    }

    println!(
        "Square Roots: sqrt({}) = {:.2}, sqrt({}) = {:.2}",
        input_one, square_root(input_one),
        input_two, square_root(input_two)
    );
}
