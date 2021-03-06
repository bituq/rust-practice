use std::{env::Args, thread::{self, JoinHandle}, time::Duration};

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

    pub fn dequeue(&mut self) -> Option<Task> {
        self.done_len += 1;
        if self.queue.len() == 0 {
            return None;
        }
        return Some(self.queue.remove(0));
    }

    pub fn execute(&mut self) -> Option<Result<(), String>> {
        match self.dequeue() {
            Some(dequeued) => Some(dequeued()),
            None => None
        }
    }
}

fn main() {
    let mut s: Queue = Queue::new();

    s.enqueue(Box::new(|| {
        println!("Waiting 3 seconds...");
        thread::sleep(Duration::from_secs(3));

        let mut threads: Vec<JoinHandle<()>> = Vec::new();

        for i in 1..4 {
            threads.push(thread::spawn(move || {
                thread::sleep(Duration::from_secs(i));
                println!("Doing work on thread {}", i);
            }));
        }

        for thread in threads {
            thread.join().unwrap();
        }

        println!("Waited for 3 seconds! Executing next thing...");
        Ok(())
    }));
    s.enqueue(Box::new(|| {
        println!("I am the next thing. Hi!");
        Ok(())
    }));

    while true {
        match s.execute() {
            Some(d) => {
                match d {
                    Ok(()) => (),
                    Err(reason) => println!("Error: {}", reason)
                };
            },
            None => break
        };
    }
}