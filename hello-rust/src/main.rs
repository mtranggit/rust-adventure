use std::io;
fn main() {
    println!("Please enter you name: ");
    let mut name = String::new();
    io::stdin().read_line(&mut name);
    say_name(&name);
}

fn say_name(name: &String) {
    println!("Hello, {}", name);
}
