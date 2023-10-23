use std::io;
use std::process;

fn main() {
    // call lib
    hello_rust::say_hello();
    hello_rust::print_one_to_five();
    loop {
        println!("Please enter your first number: ");
        let a = read_input();

        println!("Please enter your first number: ");
        let b = read_input();

        let result = sum(a, b);
        println!("{} + {} = {}", a, b, result);
    }
}

fn sum(first: u32, second: u32) -> u32 {
    first + second
}

fn read_input() -> u32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let digit: u32;
    match input.trim().parse() {
        Ok(val) => digit = val,
        Err(_err) => {
            println!("This is not a valid number");
            process::exit(1);
        }
    }

    digit
}
