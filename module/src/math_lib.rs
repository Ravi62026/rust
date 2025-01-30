pub mod math {
    pub fn add(a: u8, b: u8) -> u8 {
        a + b
    }
    pub fn sub(a: u8, b: u8) -> u8 {
        a - b
    }
}

// use math::add;
// use math::sub;

use math::*;

fn main() {
    let add_res = add(2,3);
    println!("{add_res}");
    let sub_res = sub(4,2);
    println!("{sub_res}");
}