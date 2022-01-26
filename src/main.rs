use std::io;

fn main() {
    //Convert temperatures between Fahrenheit and Celsius.

    loop {
        println!("Temperature converter!");

        let mut input = String::new();
        let mut mode = String::new();

        println!("Enter a mode they are: f2c and c2f");
        io::stdin()
            .read_line(&mut mode)
            .expect("Enter a valid input");

        let mode = mode.trim();

        println!("Enter a value");
        io::stdin()
            .read_line(&mut input)
            .expect("Enter a valid input");

        let input = match input.trim().parse() {
            Ok(val) => val,
            Err(_) => continue,
        };

        let converted = covert_temp(input, &mode);
        println!("{} degrees {}", converted, mode);
    }
}

fn covert_temp(convert: i128, mode: &str) -> i128 {
    match mode {
        "f2c" => (convert - 32) * 5 / 9,
        "c2f" => (convert * 9 / 5) + 32,
        _ => 0000,
    }
}
