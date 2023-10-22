use std::io;
use std::process;

fn main() {
    println!("Please enter your first number: ");
    let mut first = String::new();
    io::stdin().read_line(&mut first).unwrap();

    let a: u32;
    match first.trim().parse() {
        Ok(val) => {
            a = val;
        }
        Err(_err) => {
            println!("This is not a valid number");
            process::exit(1);
        }
    }

    println!("Please enter your second number: ");
    let mut second = String::new();
    io::stdin().read_line(&mut second).unwrap();

    let b: u32;
    match second.trim().parse() {
        Ok(val) => {
            b = val;
        }
        Err(_err) => {
            println!("This is not a valid number");
            process::exit(1);
        }
    }
    let result = sum(a, b);
    println!("{} + {} = {}", a, b, result);
}

fn sum(first: u32, second: u32) -> u32 {
    first + second
}
