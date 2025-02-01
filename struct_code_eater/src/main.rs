#[derive(Debug)]
struct Student {
    name: String,
    reg_no: String,
    age: u8,
    gender: String,
    department: String,
    year_of_study: u8,
    cgpa: f32,
}

struct Rectangle {
    length: u32,
    width: u32,
}

fn rect_area(rect: &Rectangle) -> u32 {
    rect.length * rect.width
}

fn update_len(rec: &mut Rectangle) {
    rec.length = 19;
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.width
    }

    fn perimeter(&self) -> u32 {
        self.length * 2 + self.width * 2
    }
}

fn main() {
    let student1 = Student {
        name: String::from("Ravi"),
        reg_no: String::from("22MEI10050"),
        age: 20,
        gender: String::from("Male"),
        department: String::from("Computer Science"),
        year_of_study: 3,
        cgpa: 9.5,
    };

    let student2 = Student {
        name: String::from("Raunak"),
        ..student1
    };

    // println!("{:#?}", student1);
    println!("{:#?}", student2);

    let mut rect = Rectangle {
        length: 10,
        width: 20,
    };

    let area = rect_area(&rect);
    println!("Area of the rectangle is {}", area);
    println!("{:#?}", rect.length);
    update_len(&mut rect);
    println!("{:#?}", rect.length);
    println!("Area via impl: {:#?}", rect.area());
    println!("Area via method: {:#?}", rect.area());
    println!("Perimeter via impl: {:#?}", rect.perimeter());
}
