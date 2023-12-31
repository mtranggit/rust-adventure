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

    // matching literals
    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // matching named variables
    let x = None;
    // let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {y}"),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {y}", x);

    // Multiple Patterns
    let x = 2;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // matching ranges of values with ..=
    let x = 5;
    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }
    let x = 'c';
    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    // Destructuring Structs
    let p = Point { x: 0, y: 7 };
    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    let q = Point { x: 0, y: 7 };
    match q {
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => {
            println!("On neither axis: ({x}, {y}) ");
        }
    }

    // Destructing Enums
    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));
    // let msg = Message::Move { x: 3, y: 7 };
    // let msg = Message::Write(String::from("Hello"));
    // let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure");
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {x} and in the y direction {y}");
        }
        Message::Write(text) => {
            println!("Text message: {text}");
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change color to hue {h}, saturation {s}, value {v}");
        }

        _ => (),
        // Message::ChangeColor(r, g, b) => {
        //     println!("Change the color to red {r}, green {g}, and blue {b}");
        // }
    }
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}

struct Point {
    x: i32,
    y: i32,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
    // ChangeColor(i32, i32, i32),
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}
