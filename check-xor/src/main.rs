use std::{fs::File, io::Read};

fn main() {
    let mut file = File::open("input.txt").expect("Failed to open file");
    let mut buffer: Vec<u8> = Vec::new();
    let mut checksum: u8 = 0;
    file.read_to_end(&mut buffer).expect("Failed to read file");
    let now = std::time::Instant::now();
    buffer.iter().for_each(|&byte| checksum ^= byte);
    println!("Time taken: {:?}", now.elapsed());
    println!("Checksum: {:x}", checksum);
}
