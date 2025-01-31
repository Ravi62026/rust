// convert a decimal number to a binary number
// convert a decimal number to a hexadecimal number
// convert a decimal number to a octal number
// convert a binary number to a decimal number
// convert a binary number to a hexadecimal number
// convert a binary number to a octal number
// convert a hexadecimal number to a decimal number
// convert a hexadecimal number to a binary number
// convert a octal number to a decimal number
// convert a octal number to a binary number
// convert a octal number to a hexadecimal number

use std::io;

fn get_input(prompt: &str) -> u32 {
    loop {
        println!("{}", prompt);
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("error in reading input");

        match input.trim().parse::<u32>() {
            Ok(num) => return num,
            Err(_) => {
                print!("Please enter a valid number");
                continue;
            }
        }
    }
}

fn binary_converter(num: u32) {
    let mut binary = String::new();
    let mut n = num;

    while n > 0 {
        let rem = (n % 2) as u8;
        binary.insert(0, (rem + 48) as char);
        n /= 2;
    }

    println!("Binary: {}", binary);

}

fn hexadecimal_converter(num: u32) {
    let mut hexadecimal = String::new();
    let mut n = num;

    while n > 0 {
        let rem: u8 = (n % 16) as u8;

        if rem > 9 {
            hexadecimal.insert(0, (rem + 55) as char);
        } else {
            hexadecimal.insert(0, (rem + 48) as char);
        }
        n /= 16;
    }

    println!("Hexadecimal: {}", hexadecimal);
}

fn main() {
    println!("Hello, world!");
    let num = get_input("Enter a number: ");
    println!("You entered: {}", num);

    binary_converter(num);
    println!("\n");
    hexadecimal_converter(num);
}
