// fn main() {
//     let s1 = String::from("hello");

//     let len = calculate_length(&s1);

//     println!("The length of '{s1}' is {len}.");
// }

// fn calculate_length(s: &String) -> usize {
//     s.len()
// }

// fn main() {
//     let mut  s = String::from("hello");

//     s.push_str("Inside main");

//     change(&s);
// }

// fn change(some_string: &String) {
//     some_string.push_str(", world");  // cannot borrow `*some_string` as mutable, as it is behind a `&` reference `some_string` is a `&` reference, so the data it refers to cannot be borrowed as mutable
// }

// fn main() {
//     let mut s = String::from("hello");

//     s.push_str(" Hi");

//     change(&mut s);

//     print!("{s}");
// }

// fn change(s: &mut String) {
//     s.push_str(", world");
// }

// fn main() {
//     let mut s = String::from("hello");

//     let r1 = &mut s;

//     let r2 = s; // allow 
//     r1.push_str(" world");
    
//     // let r2 = &mut s; // not allowed

//     println!("{}", r1);

// }


// fn main() {
//     let mut s = String::from("Hello");

//     {
//         let r1 = &mut s;
//         let r2 = s;  // value moved here 

//         print!("{r1}");
//     }

//     let r2 = &mut s;  // borrow of moved value: `s`
// }

// fn main() {
//     let mut s = String::from("hello");

//     {
//         let r1 = &mut s;
//     } // r1 goes out of scope here, so we can make a new reference with no problems.

//     let r2 = &mut s;
// }

// fn main() {
//     let mut s = String::from("hello");

//     let r1 = &s; // no problem
//     let r2 = &s; // no problem
//     let r3 = &mut s; // BIG PROBLEM

//     println!("{}, {}, and {}", r1, r2, r3);

// }

// fn main() {
//     let mut s = String::from("hello");

//     let r1 = &s; // no problem
//     let r2 = &s; // no problem
//     println!("{r1} and {r2}");
//     // variables r1 and r2 will not be used after this point

//     let r3 = &mut s; // no problem
//     println!("{r3}");
// }


// DANGLING REFERENCES

fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
}