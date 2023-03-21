use std::thread;
use std::time::Duration;

pub fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("Count in thread: {i}!");
            thread::sleep(Duration::from_millis(5));
        }
    });

    for i in 1..5 {
        println!("Count from main thread: {i}");
        thread::sleep(Duration::from_millis(5));
    }
}
