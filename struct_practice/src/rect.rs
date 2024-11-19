

// fn main() {
    
//     // let w = 100;
//     // let h = 200;

//     let rec = (30,40);

//     // let area = calculate_area(w,h);

//     let area = calculate_area_2(rec);

//     println!("The area is {}", area);
// }

// fn calculate_area(width: u32, height: u32) -> u32 {
//     width*height
// }

// fn calculate_area_2(dimension: (u32, u32)) -> u32 {
//     let (width, height) = dimension;
//     width*height
// }

#[derive(Debug)]
struct Rec {
    width: u32,
    height: u32,
}

fn main() {
    let rect = Rec {
        width : 30,
        height : 68,
    };

    let area = calculate_area(&rect);

    dbg!(&rect);

    println!("The area of {:#?} is  {}", rect, area);
}

fn calculate_area(rect: &Rec) -> u32 {
    rect.width*rect.height
}