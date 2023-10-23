pub fn say_hello() {
    println!("Hello, world!");
}

pub fn print_one_to_five() {
    let numbers = [1, 2, 3, 4, 5];
    for n in numbers.iter() {
        println!("Printing {}", n);
    }
}
