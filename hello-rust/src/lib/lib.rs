use std::collections::HashMap;

pub fn say_hello() {
    println!("Hello, world!");
}

pub fn print_one_to_five() {
    let numbers = [1, 2, 3, 4, 5];
    for n in numbers.iter() {
        println!("Printing {}", n);
    }
}

// example of lifetime
pub fn get_city<'a, 'b>(code: &'a str, airport_codes: &'b HashMap<&str, &str>) -> &'b str {
    airport_codes
        .get(code)
        .expect("We don't know this location!")
}
