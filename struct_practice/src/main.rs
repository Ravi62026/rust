#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }

    fn square(side:u32)-> Rectangle{
        Rectangle{
            width:side,
            height:side,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rec2 = Rectangle {
        width: 90,
        height: 38,
    };

    // rec2.area();

    // println!(
    //     "The area of the rectangle is {:#?} is {} square pixels.",
    //     rect1, rect1.area()
    // );

    // let mut s1 = String::from("Ravi Shankar");

    // s1.push_str(" how are u ?");

    // println!("Can rect 1 hold rect 2? {}", rec2.can_hold(&rect1))

    let square = Rectangle::square(5);

    println!("{:#?}", square);
}