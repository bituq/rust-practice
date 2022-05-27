use std::{env::Args, thread, time::Duration};

type Task = Box<dyn Fn() -> Result<(), String>>;

struct Queue {
    pub queue: Vec<Task>,
    pub max_len: usize,
    pub done_len: usize,
}

impl Queue {
    pub fn new() -> Self {
        Self { queue: Vec::new(), done_len: 0, max_len: 0 }
    }

    pub fn enqueue(&mut self, f: Task) {
        self.max_len += 1;
        self.queue.push(f);
    }

    pub fn dequeue(&mut self) -> Task {
        self.done_len += 1;
        self.queue.remove(0)
    }

    pub fn execute(&mut self) {
        match self.dequeue()() {
            Err(reason) => println!("Error: {}", reason),
            _ => ()
        };
    }
}

fn main() {
    let mut s: Queue = Queue::new();

    s.enqueue(Box::new(|| {
        println!("Waiting 3 seconds...");
        thread::sleep(Duration::from_secs(3));
        println!("Waited for 3 seconds! Executing next thing...");
        Ok(())
    }));
    s.enqueue(Box::new(|| {
        println!("I am the next thing. Hi!");
        Err("UNKNOWN LOL".into())
    }));
    s.execute();
    s.execute();
}