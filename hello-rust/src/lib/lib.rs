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

pub trait Repo {
    fn get_clone_url(&self) -> String;
}

pub fn build<T: Repo>(repo: T) {
    let url = repo.get_clone_url();
    println!("Cloning {}", url);
}

pub fn get_file_extension(path: impl AsRef<str>) -> String {
    let path = path.as_ref();
    String::from(path.split(".").last().unwrap())
}
