use std::thread;
use std::time::Duration;

fn main() {
    let v = vec![1,2,3];

    let handle = thread::spawn(move || {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            println!("here's a vector: {:?} moved the spawned thread!", v);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();

    for i in 1..5 {
        println!("hello number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // handle.join().unwrap();
}
