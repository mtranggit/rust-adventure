use std::io;

fn main() {
    println!("Please enter your first number: ");
    let mut first = String::new();
    io::stdin().read_line(&mut first).unwrap();

    let mut a: u32 = 0;
    match first.trim().parse() {
        Ok(val) => {
            a = val;
        }
        Err(_err) => {
            println!("This is not a valid number");
        }
    }

    println!("Please enter your second number: ");
    let mut second = String::new();
    io::stdin().read_line(&mut second).unwrap();

    let mut b: u32 = 0;
    match second.trim().parse() {
        Ok(val) => {
            b = val;
        }
        Err(_err) => {
            println!("This is not a valid number");
        }
    }
    let result = sum(a, b);
    println!("{} + {} = {}", a, b, result);
}

fn sum(first: u32, second: u32) -> u32 {
    first + second
}
