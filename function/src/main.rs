fn main() {
    println!("Hello, world!");

    my_function();

    with_para("Ravi");

    let ans = add(3,4);
    println!("Result is : {}", ans);


}

fn my_function() {
    println!("Hello from my function");
}

fn with_para (name: &str) {
    println!("Hello {}", name);
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}