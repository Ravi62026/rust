fn main() {
    // let x = 5;

    // if x < 3 {
    //     println!("incorrect");
    // }
    // else{
    //     println!("Correct")
    // }

    // if y is true x = 10 else x = 20

    // let y = is_even(4);

    // let x = if y {10} else {20};
    // println!("x = {x}");

    // let number = 6;

    // if number % 4 == 0 {
    //     println!("number is divisible by 4");
    // } else if number % 3 == 0 {
    //     println!("number is divisible by 3");
    // } else if number % 2 == 0 {
    //     println!("number is divisible by 2");
    // } else {
    //     println!("number is not divisible by 4, 3, or 2");
    // }


    // Repetition with Loops

    // 1. loop

    // let mut num = 1;
    // loop {
    //     println!("HI");

    //     if num == 5{
    //         continue;
    //     }
    //     num = num + 1;

    //     if num == 10 {
    //         break;
    //     }

    //     // num = num + 1;
    // }

    // loop label

    // let mut count = 0;

    // 'counting_up: loop {
    //     println!("count = {count}");
    //     let mut remaining = 10;

    //     loop {
    //         println!("remaining = {remaining}");
    //         if remaining == 9 {
    //             break;
    //         }
    //         if count == 2 {
    //             break 'counting_up;
    //         }
    //         remaining -= 1;
    //     }

    //     count += 1;
    // }
    // println!("End count = {count}");

    // WHILE LOOP


    // let mut num = 3;

    // while num !=0 {
    //     println!("{num}");

    //     num = num - 1;
    // }

    // print!("LIFTOFF!!!!");

    // let a = [10, 20, 30, 40, 50];
    // let mut index = 0;

    // while index < 5 {
    //     println!("the value at index {index} is: {}", a[index]);

    //     index += 1;
    // }


    // FOR LOOP

    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");


}


// fn is_even(x:i32) -> bool {
//     if x % 2 == 0 {
//         return true;
//     }
//     return false;
// }