use std::io;

fn main() {
    println!("Please enter your first number: ");
    let mut first = String::new();
    io::stdin().read_line(&mut first);

    let a: u32 = first.trim().parse().unwrap();

    println!("Please enter your second number: ");
    let mut second = String::new();
    io::stdin().read_line(&mut second);

    let b: u32 = second.trim().parse().unwrap();
    let result = sum(a, b);
    println!("{} + {} = {}", a, b, result);
}

fn sum(first: u32, second: u32) -> u32 {
    first + second
}
