use std::{env, process};

use minigrep::{run, Config};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Search for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(err) = run(config) {
        eprintln!("Application error: {err}");
        process::exit(1);
    };
}
