use std::time::Instant;
use std::{env::args, process::exit};

// Retreived using getconf PAGESIZE
const PAGE_SIZE: usize = 4096;
const JUMP_SIZE: usize = PAGE_SIZE / size_of::<i32>();

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() != 3 {
        println!("Usage: pages iterations");
        exit(1);
    }
    let pages: usize = args[1].parse().unwrap();
    let iterations: usize = args[2].parse().unwrap();
    println!("Pages: {} Iterations: {}", pages, iterations);
    let mut array = vec![0i32; (PAGE_SIZE * pages) / size_of::<i32>()];
    let start = Instant::now();
    for _ in 0..iterations {
        for i in (0..array.len()).step_by(JUMP_SIZE) {
            array[i] += 1;
        }
    }
    println!("Random element: {}", array[JUMP_SIZE]);
    println!(
        "Trial time: {:?}",
        (start.elapsed() / iterations as u32) / (array.len() as u32 / JUMP_SIZE as u32)
    );
}
