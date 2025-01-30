struct user {
    active:bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct color (u8, u8, u8);
struct point (u8, u8, u8);

struct AlwaysEqual;
fn main() {
    
    let user1 = user{
        active: true,
        username: String::from("Ravi"),
        email: String::from("ravi@gmail.com"),
        sign_in_count:1,
    };

    let user2 = user{
        active: user1.active,
        username: String::from("Raunak"),
        email: String::from("raunak@gmail.com"),
        sign_in_count:1,
    };

    // user2.email = String::from("Hello"); // immutable error

    println!("The name of user is {}", user1.username);

    let mut user3 = user{
        active: true,
        username: String::from("Raunak"),
        email: String::from("raunak@gmail.com"),
        sign_in_count:1,
    };

    println!("The name of user3 befor mutate is: {}", user1.username);

    user3.username = String::from("VIT");

    println!("The name of user3 befor mutate is {}", user1.username);

    let user4 = build_user(
        String::from("Ravi Shankar"),
        String::from("hello@gmail.com"),
    );

    println!("Value of username of user4 is {} ", user4.username);

    let user5 = user{
        username: String::from("Hello"),
        ..user4
    };

    // println!("is user 4 {}", user4.email); // borrowing error as ownership transfer to user 5

    // STRUCT TUPLE

    let red : color = color(100, 0, 0);
    set_bg_color(red);

    let coor: point = point(30,40,20);
    move_point(coor);

    // UNIT LIKE STRUCT WITHOUT ANY FIELD

    let subject = AlwaysEqual;

    


}

fn build_user(username: String, email: String) -> user {
    user {
        username: username,
        email: email,
        active: true,
        sign_in_count: 0,
    }
}

fn set_bg_color(colour: color) {
    println!("Setting bg color R = {}, G = {}, B = {} ", colour.0, colour.1, colour.2);
}

fn move_point(points : point) {
    println!("Thecursor was moved X = {}, Y = {}, Z = {} ", points.0, points.1, points.2);
}

