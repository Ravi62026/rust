// user will guess a number
//random function will generate a random value
// guessed nu < random val -> gn is less than rv
// guessed nu > random val -> gn is greater than rv
// guessed nu == random val -> gn is equal to rv

use rand::Rng;
use std::io;

fn get_input(prompt: &str) -> u32{
    loop {
        let mut input = String::new();

        io::stdin()
        .read_line(&mut input)
        .expect("error in reading input");

        match input.trim().parse::<u32>(){
            Ok(num) => return num,
            Err(_) => {
                println!("{}", prompt);
                continue;
            }
        }
    }
}

fn generate_random () -> u32 {
    let secret_number = rand::thread_rng().gen_range(1..=10);
    return secret_number
}

fn main (){
    println!("Guess Your Number: ");
    
    let random_number = generate_random();
    println!("Random number is {}", random_number);

    loop {
        let guessed_number = get_input("Not a number");

        // if guessed_number < random_number {
        //     println!("Guessed number is smaller")
        // }else if guessed_number > random_number {
        //     println!("Guessed number is greater")
        // }else {
        //     println!("You Won!");
        //     break;
        // }

        match guessed_number.cmp(&random_number) {
            std::cmp::Ordering::Greater => println!("Guess is greater"),
            std::cmp::Ordering::Less => println!("Guess is less"),
            std::cmp::Ordering::Equal => {
                println!("You won!");
                break;
            },
        }
    }
}