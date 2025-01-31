// create a banking aaplication
// which takes input from the user
// and performs the following operations
// 1. deposit
// 2. withdraw
// 3. check balance
// 4. exit

// the user should be able to perform these operations
// multiple times until they choose to exit

// the balance should be stored in a variable
// and should be updated after each transaction

// the program should handle invalid inputs gracefully
// and should prompt the user to enter a valid input

// the program should display the current balance after each transaction

use core::f64;
use std::io;
use::std::thread;
use std::time::Duration;

fn get_input() -> u32 {
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse::<u32>() {
            Ok(num) => return num,
            Err(_) => {
                println!("Please enter a valid number");
                continue;
            }
        }
    }
}

fn deposit(user_balance: &mut f64) {
    println!("Enter the amount to deposit");
    let amount = get_input();
    *user_balance += amount as f64;
    println!("Deposited {} successfully", amount); 
}

fn withdraw(user_balance: &mut f64) {
    println!("Enter the amount to withdraw");
    let amount = get_input();
    *user_balance -= amount as f64;
    println!("Withdrawn {} successfully", amount);
}

fn check_balance(user_balance: &mut f64) {
    println!("Your balance is {}", user_balance);
}

fn exit() {
    println!("Thanks for using our services");
}

fn main() {
    println!("Hello, world!");
    println!("Welcome to the banking application");
    let mut user_balance = 0.0;

    loop {
        println!("Enter your choice");
        println!("1. Deposit");
        println!("2. Withdraw");
        println!("3. Check balance");
        println!("4. Exit");
        let choice = get_input();

        if choice == 4 {
            exit();
            break;
        }
        execute(choice, &mut user_balance);
        thread::sleep(Duration::from_secs(1));
    }
    
}

fn execute(choice: u32, user_balance: &mut f64) {
    match choice {
        1 => deposit(user_balance),
        2 => withdraw(user_balance),
        3 => check_balance(user_balance),
        4 => exit(),
        _ => println!("Invalid choice"),
    }
}