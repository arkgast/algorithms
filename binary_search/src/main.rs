use std::io;
use std::io::Write;

fn main() {
    let number = read_number_input();
    println!("Number read: {}", number);
}

fn read_number_input() -> i32 {
    print!("Enter a number: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let number = parse_input_to_number(input);
    return number;
}

fn parse_input_to_number(input_value: String) -> i32 {
    let mut number: i32 = 0;
    match input_value.trim().parse() {
        Ok(parsed_number) => number = parsed_number,
        Err(error) => {
            eprintln!("> Error parsing number {}", error);
            eprintln!("> {:?}", error);
        }
    }
    return number;
}
