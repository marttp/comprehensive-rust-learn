use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct Fork;

struct Philosopher {
    name: String,
    left_fork: Arc<Mutex<Fork>>,
    right_fork: Arc<Mutex<Fork>>,
    thoughts: mpsc::SyncSender<String>,
}

impl Philosopher {
    fn think(&self) {
        self.thoughts
            .send(format!("Eureka! {} has a new idea!", &self.name))
            .unwrap();
    }

    fn eat(&self) {
        // Pick up forks... => Lock resource
        println!("{} try to eat something", &self.name);
        self.left_fork.lock().unwrap();
        self.right_fork.lock().unwrap();

        // Use fork
        println!("{} is eating...", &self.name);
        thread::sleep(Duration::from_millis(10));
    }
}

static PHILOSOPHERS: &[&str] = &["Socrates", "Plato", "Aristotle", "Thales", "Pythagoras"];

pub fn main() {
    let (tx, rx) = mpsc::sync_channel(5);
    let len = PHILOSOPHERS.len();
    // Create forks
    let forks = (0..len)
        .map(|_| Arc::new(Mutex::new(Fork)))
        .collect::<Vec<_>>();

    for i in 0..forks.len() {
        let tx = tx.clone();
        // Circle shape table
        let mut left_fork = forks[i].clone();
        let mut right_fork = forks[(i + 1) % forks.len()].clone();
        // avoid deadlock - copy from solution
        if i == forks.len() - 1 {
            std::mem::swap(&mut left_fork, &mut right_fork);
        }
        // Create philosophers
        let philosopher = Philosopher {
            name: PHILOSOPHERS[i].to_string(),
            thoughts: tx,
            left_fork,
            right_fork,
        };
        // Make them think and eat
        thread::spawn(move || {
            for _ in 0..100 {
                philosopher.think();
                philosopher.eat();
            }
        });
    }

    // Output their thoughts
    drop(tx);
    for thought in rx {
        println!("{thought}");
    }
}
