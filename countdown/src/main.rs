// countdown rust

// input -> 10
// output -> 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0

use std::io;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    loop {
        let mut input = String::new();
        println!("Enter a number to start the countdown: ");
        io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

        let timer : u32 = match input.trim().parse(){
            Ok(time) => time,
            Err(_) => {
                println!("Please enter a valid number");
                continue;
            }
        };

        start_timer(timer);

        println!("Blast off!");
        break;
    }
    
}

fn start_timer(timer: u32) {

    for i in (0..=timer).rev() {
        println!("Timer Countdown.. {}", i);
        sleep(Duration::from_secs(1));
    }
}
