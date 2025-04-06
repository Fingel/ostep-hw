use std::sync::{Arc, RwLock};

fn main() {
    let vec: Arc<RwLock<Vec<usize>>> = Arc::new(RwLock::new(Vec::new()));
    let num_threads = 10;

    let mut handles: Vec<std::thread::JoinHandle<()>> = Vec::new();
    for i in 0..num_threads {
        let vec = vec.clone();
        let handle = std::thread::spawn(move || {
            vec.write().unwrap().push(i);
        });
        handles.push(handle);
    }
    for _ in 0..10 {
        let r_val = vec.clone();
        std::thread::spawn(move || {
            let val = r_val.read().unwrap();
            println!("read: {:?}", val);
        });
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("{:?}", vec.read().unwrap());
}
