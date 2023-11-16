use add_one;

fn main() {
    let num = 10;
    println!("Hello, world! {num} plus one is {}!", add_one::add_one(num));
    println!(
        "Random number between 1 and 100 is: {}!",
        add_one::random_number_between_1_and_100()
    );
    println!("{num} plus five is {}!", add_five::add_five(num));
}
