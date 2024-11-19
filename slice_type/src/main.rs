// fn main() {
//     let mut str = String::from("Hello World");

//     let res = find_first_space(&str);

//     // str.clear();

//     println!("Result is {res} ");
// }

// fn find_first_space(input: &String) -> usize {
//     let s = input.as_bytes();

//     for (i, &item) in s.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }

//     s.len()
// }


// STRING SLICES

// fn main() {
//     let mut s = String::from("Hello jii");

//     let hello = &s[0..5];
//     let world = &s[6..9];

//     // s.clear();

//     println!("Hello = {hello} ");
//     println!("World = {world} ");
// }

// fn main() {
//     let str = String::from("Hello World");

//     let res = find_first_space(&str);

//     // str.clear();

//     println!("Result is {res} ");
// }

// fn find_first_space(input: &String) -> &str {
//     let s = input.as_bytes();

//     for (i, &item) in s.iter().enumerate() {
//         if item == b' ' {
//             return &input[..i];
//         }
//     }

//     &input[..]
// }

// fn main() {
//     let s = [1, 2, 3];
//     for (i, item) in s.iter().enumerate() {
//         // item is of type &i32, not i32
//         println!(" {i}, {item}");
//     }

// }

// fn main() {
//     let str = String::from("Hello World");

//     let res = first_word(&str);

//     // str.clear();

//     println!("Result is {res} ");
// }

// fn first_word(s: &String) -> &str {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[0..i];
//         }
//     }

//     &s[..]
// }


// fn main() {
//     let my_string = String::from("hello world");

//     // `first_word` works on slices of `String`s, whether partial or whole
//     let word = first_word(&my_string[0..6]);
//     let word = first_word(&my_string[..]);
//     // `first_word` also works on references to `String`s, which are equivalent
//     // to whole slices of `String`s
//     let word = first_word(&my_string);

//     let my_string_literal = "hello world";

//     // `first_word` works on slices of string literals, whether partial or whole
//     let word = first_word(&my_string_literal[0..6]);
//     let word = first_word(&my_string_literal[..]);

//     // Because string literals *are* string slices already,
//     // this works too, without the slice syntax!
//     let word = first_word(my_string_literal);
// }


// fn first_word(s: &String) -> &str {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[0..i];
//         }
//     }

//     &s[..]
// }

fn main() {
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
}