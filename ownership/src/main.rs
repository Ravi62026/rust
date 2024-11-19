fn main() {
    // println!("Hello, world!");
    // let str = "Ravi";
    // println!("{str}");

    {
        // println!("{x}");  // cannot find value `x` in this scope

        // let x = "JII";  // x scope starts
    } // x scope ends

    // println!("x : {x} "); // annot find value `x` in this scope

    // let mut s = String::from("Ravi");

    // println!("{s}");

    // s.push_str(" Shankar");
    // s.push('a');

    // println!("{s}");

    // let mut x = 5;
    // let y = x; // copy

    // print!("Before update x ");

    // println!("x is {x} and y is {y} ");

    // x = 10;

    // print!("After update x ");

    // println!("x is {x} and y is {y} ");

    // let s1 = String::from("hello");
    // let s2 = s1; // s1 invalidated after writing this line

    // print!("Before update s1 ");

    // println!("s1 is {s1} and s2 is {s2} "); // borrow of moved value: `s1` value borrowed here after move


    // let mut s1 = String::from("hello");
    // let s2 = s1.clone();

    // print!("Before update s1 ");

    // println!("s1 = {s1}, s2 = {s2}");

    // s1.push_str("string");

    // print!("After update s1 ");

    // println!("s1 = {s1}, s2 = {s2}");

    // let num = 10;
    // let result = add(num);

    // let name = String::from("Ravi");
    // let s = gives_ownership();
    // takes_ownership(name);

    // println!("Num is {num} and result = {result} ");
    // // println!("Value of name is {name} "); // borrow of moved value: `name` because as name is pass to function the owner of name is change from name variable to function that is why give error of borrow

    // println!("S = {s}");

    // let s2 = String::from("Hi ji");

    // let s2 = gives_take_ownership(s2);

    // println!("s2 = {s2}");

    let s = String::from("JAY BABA PANCHBADAN");
    // let l = calculate_length(s);

    let (s1, len) = calculate_length(s);

    println!("The length of {s1} is {len} ");  // move borrowed

}

// fn takes_ownership(s: String) {
//     println!("Inside Ownership {s} ");
// }
 
// fn add (x : i32) -> i32 {
//     x + 10
// }

// fn gives_ownership() -> String {
//     let s = String::from("This is a string from gives ownership");
//     s
// }

// fn takes_ownership(s: String) {
//     println!("Inside Ownership {s} ");
// }

// fn gives_take_ownership(s: String) -> String{
//     println!("give and take {s}");
//     s
// }


fn calculate_length(s: String) -> (String, usize) {
    let res = s.len();
    (s, res)
}