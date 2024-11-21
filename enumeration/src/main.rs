// #[derive(Debug)]
// enum IpAddrKind {
//     V4,
//     V6,
// }

// #[derive(Debug)]
// struct ip {
//     address: String,
//     kind: IpAddrKind
// }

// #[derive(Debug)]
// enum IpAddr {
//     V4(String),
//     V6(String)
// }

// #[derive(Debug)]
// enum IpAddrWithTuple {
//     V4(u8, u8, u8, u8),
//     V6(String),
// }

// #[derive(Debug)]
// struct Ipv4Addr {
//     address: (u8, u8, u8, u8),
// }

// #[derive(Debug)]
// struct Ipv6Addr {
//     address: String, // IPv6 addresses are typically represented as a string.
// }

// #[derive(Debug)]
// enum IpAddrWithStruct {
//     V4(Ipv4Addr),
//     V6(Ipv6Addr),
// }



// fn main() {
//     println!("Hello, world!");

//     // Example usage of routes function
//     // let ip = String::from("192.168.0.1");
//     // let kind = IpAddrKind::V4;
//     // routes(ip, kind);

//     // let home = ip {
//     //     address: String::from("127.0.0.1"),
//     //     kind: IpAddrKind::V4
//     // };

//     // let loophole = ip {
//     //     address: String::from("::1"),
//     //     kind: IpAddrKind::V6
//     // };

//     // routes(home);

//     // let home = IpAddr::V6(String::from("::1"));
//     // routes(home);

//     // let home = IpAddrWithTuple::V4(127,0,0,1);

//     // routes(home);

//     let ipv4 = Ipv4Addr {
//         address: (192, 168, 0, 1),
//     };

//     let ip_addr_v4 = IpAddrWithStruct::V4(ipv4);
//     display_ip(ip_addr_v4);

//     let ipv6 = Ipv6Addr {
//         address: String::from("2001:0db8:85a3:0000:0000:8a2e:0370:7334"),
//     };

//     let ip_addr_v6 = IpAddrWithStruct::V6(ipv6);

//     display_ip(ip_addr_v6);


// }

// // fn routes(ip: String, kind: IpAddrKind) {
// //     println!("The IP address is {ip} and kind is {:?}", kind);
// // }

// // fn routes(home: ip) {
// //     println!("The IP is {:?}", home);
// // }

// // fn routes(home: IpAddr) {
// //     println!("The IP is {:?}", home);
// // }

// // fn routes(home: IpAddrWithTuple) {
// //     println!("The IP is {:?}", home);
// // }
// // fn routes(home: IpAddrWithStruct) {
// //     println!("The IP is {:?}", home);
// // }

// fn display_ip(ip: IpAddrWithStruct) {
//     match ip {
//         IpAddrWithStruct::V4(ipv4) => {
//             let (a, b, c, d) = ipv4.address;
//             println!("IPv4 address: {}.{}.{}.{}", a, b, c, d);
//         }
//         IpAddrWithStruct::V6(ipv6) => {
//             println!("IPv6 address: {}", ipv6.address);
//         }
//     }
// }



// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }

// impl Message {
//     fn quit() {
//         println!("Quit: Exiting the application.");
//     }

//     fn move_to(x: i32, y: i32) {
//         println!("Move: Moving to position ({}, {}).", x, y);
//     }

//     fn write_message(text: &str) {
//         println!("Write: Displaying message '{}'.", text);
//     }

//     fn change_color(r: i32, g: i32, b: i32) {
//         println!("ChangeColor: Changing color to RGB({}, {}, {}).", r, g, b);
//     }
// }

enum Option<T> {
    None,
    Some(T),
}

fn main() {
    // Call each function directly
    // Message::quit();

    // Message::move_to(10, 20);

    // Message::write_message("Hello, Rust!");

    // Message::change_color(255, 100, 50);

    // let some_number = Some(5);
    // let some_char = Some('e');

    // let absent_number: Option<i32> = None;  // mismatched types `std::option::Option<_>` and `Option<i32>` have similar names, but are actually distinct 

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y;
}

