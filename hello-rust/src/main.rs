fn main() {
    let first = "Michael".to_string();
    let last = "Trang".to_string();
    say_name(first, last);
}

fn say_name(first: String, last: String) {
    println!("Hello, {} {}!", first, last);
}
