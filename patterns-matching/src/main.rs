fn main() {
    // explore if let
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "29".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color} as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }

    // explore while let
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);
    stack.push(4);
    stack.push(5);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    // explore for loops
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    // explore let statements
    let i = 5;
    let (x, y, z) = (1, 2, 3);
    println!("i is {}", i);
    println!("x = {}, y = {} and z = {} ", x, y, z);

    // explore function parameters
    let point = (3, 5);
    print_coordinates(&point);
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}
