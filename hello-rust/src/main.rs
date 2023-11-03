use core::{num, panic};
use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result};
use std::fs::File;
use std::io::{self, ErrorKind};
use std::process;

use hello_rust::{build, get_file_extension, get_largest};
use hello_rust::{get_city, Repo};

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

    // trait
    let github = GitHub {
        owner: "rust-lang".to_string(),
        repo: "rust".to_string(),
    };

    let gitlab = GitLab {
        user: "rust-lang".to_string(),
        repo: "rust".to_string(),
    };

    // it is now possible to print github, gitlab after impl Display
    println!("Printing GitHub {}", github);
    println!("Printing GitLab {}", gitlab);

    // call build from lib
    build(github);
    build(gitlab);

    let string_input = String::from("./lib/lib.rs");
    let extension = get_file_extension(&string_input);
    println!("The file extension of {} is {}", string_input, extension);

    let mut repos = Repositories {
        values: vec![String::from("a"), String::from("b"), String::from("c")],
    };
    add_repository(&mut repos, String::from("d"));

    println!("{:?}", repos.values);

    // explore HashMap
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    for (key, value) in scores {
        println!("{key}: {value}");
    }

    let number_list = vec![34, 33, 50, 22, 55, 100, 65];
    let result = get_largest(&number_list);
    println!("The largest number is {}", result);

    // explore error handling
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        // Err(error) => panic!("Problem opening the file: {:?}", error),
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };

    let goodbye_file = File::open("goodbye.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("goodbye.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

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

#[derive(Debug)]
struct GitHub {
    owner: String,
    repo: String,
}

#[derive(Debug)]
struct GitLab {
    user: String,
    repo: String,
}

impl Repo for GitHub {
    fn get_clone_url(&self) -> String {
        format!("https://github.com/{}/{}.git", self.owner, self.repo)
    }
}

impl Repo for GitLab {
    fn get_clone_url(&self) -> String {
        format!("https://gitlab.com/{}/{}.git", self.user, self.repo)
    }
}

impl Display for GitHub {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "GitHub: {}", self.get_clone_url())
    }
}

impl Display for GitLab {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "GitLab: {}", self.get_clone_url())
    }
}

struct Repositories {
    values: Vec<String>,
}

fn add_repository(current: &mut Repositories, new: String) {
    let values = current.as_mut();
    values.push(new);
}

impl AsMut<Vec<String>> for Repositories {
    fn as_mut(&mut self) -> &mut Vec<String> {
        &mut self.values
    }
}
