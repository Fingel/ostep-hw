use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

struct Counter {
    count: Mutex<u32>,
}

impl Counter {
    fn new() -> Self {
        Counter {
            count: Mutex::new(0),
        }
    }

    fn increment(&self) {
        *self.count.lock().unwrap() += 1;
    }

    fn get(&self) -> u32 {
        *self.count.lock().unwrap()
    }
}

struct ApproxCounter {
    global_count: Mutex<u32>,
    local_count: Mutex<u32>,
    threshold: u32,
}

impl ApproxCounter {
    fn new(threshold: u32) -> Self {
        ApproxCounter {
            global_count: Mutex::new(0),
            local_count: Mutex::new(0),
            threshold,
        }
    }

    fn increment(&self) {
        let mut local = self.local_count.lock().unwrap();
        *local += 1;
        if *local >= self.threshold {
            let mut global = self.global_count.lock().unwrap();
            *global += *local;
            *local = 0;
        }
    }

    fn get(&self) -> u32 {
        let global = self.global_count.lock().unwrap();
        let local = self.local_count.lock().unwrap();
        *global + *local
    }
}

fn main() {
    let approx_counter = Arc::new(ApproxCounter::new(6));
    let mut handles = vec![];

    let start = Instant::now();
    for _ in 0..16 {
        let approx_counter = Arc::clone(&approx_counter);
        let handle = thread::spawn(move || {
            for _ in 0..500_000 {
                approx_counter.increment();
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    let elapsed = start.elapsed();
    println!("Approximate: {} took {:?} ", approx_counter.get(), elapsed);

    let counter = Arc::new(Counter::new());
    let mut handles = vec![];

    let start = Instant::now();
    for _ in 0..16 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..500_000 {
                counter.increment();
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    let elapsed = start.elapsed();
    println!("Precise: {} took {:?} ", counter.get(), elapsed);
}
