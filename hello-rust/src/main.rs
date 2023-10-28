use std::collections::HashMap;
use std::io;
use std::process;

use hello_rust::get_city;

fn main() {
    // call lib
    hello_rust::say_hello();
    hello_rust::print_one_to_five();
    let app = URL::from("https://app.rust-for-js.dev/posts/structs-and-methods");
    println!("{:?}", app);

    let airport_codes = HashMap::from([
        ("PIE", "St. Petersburg"),
        ("LHR", "London"),
        ("BOM", "Mumbai"),
        ("SYD", "Sydney"),
        ("MEL", "Melbourne"),
    ]);

    let me = Person {
        name: "Michael",
        location: "SYD",
    };

    let you = Person {
        name: "John",
        location: "MEL",
    };

    // let other = Person {
    //     name: "Kliff",
    //     location: "ADD",
    // };

    println!(
        "Welcome, {} from {}!",
        me.name,
        get_city(&me.location, &airport_codes)
    );
    println!(
        "Welcome, {} from {}!",
        you.name,
        get_city(&you.location, &airport_codes)
    );
    // println!(
    //     "Welcome, {} from {}!",
    //     other.name,
    //     get_city(&other.location, &airport_codes)
    // );

    // iterator
    let x = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let y: Vec<i32> = x.into_iter().filter(|a| a % 2 == 0).collect(); // collect even number
    let z = &y[0..3];
    println!("Even number {:?} from {:?}", y, x);
    println!("Subset of 3 element of y {:?}", z);

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

#[derive(Debug)]
struct URL {
    protocol: String,
    hostname: String,
    pathname: String,
}

impl URL {
    fn toString(&self) -> String {
        format!("{}://{}/{}", self.protocol, self.hostname, self.pathname)
    }
    fn from(url: &str) -> URL {
        let string = String::from(url);
        let vec: Vec<&str> = string.split("://").collect();
        let protocol = String::from(vec[0]);
        let rest = String::from(vec[1]);
        let vec2: Vec<&str> = rest.split("/").collect();
        let hostname = String::from(vec2[0]);
        let pathname = String::from(vec2[1]);
        URL {
            protocol,
            hostname,
            pathname,
        }
    }
}

struct Person<'a> {
    name: &'a str,
    location: &'a str,
}
