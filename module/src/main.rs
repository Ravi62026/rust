// mod math_lib;

// use math_lib::math::*;

// fn main() {
//     let add_res = add(2,3);
//     println!("{add_res}");
// }


fn main() {
    let x = 5;

    match x {
        // 1 => println!("one"),
        // 2 => println!("two"),
        // 3 => println!("three"),
        // _ => println!("{x}"),

        n if n == 5 => println!("five"),
        _ => println!("other"),
    }
}
