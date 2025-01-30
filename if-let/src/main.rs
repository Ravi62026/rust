fn main() {
    let config_max = Some(3_u8);

    // match config_max {
    //     Some(max) => println!("The maximum is configured to be {max} "),
    //     None => (),
    // }

    if let Some(max) = config_max{
        println!("The max is conf to be {max}");
    }
}
