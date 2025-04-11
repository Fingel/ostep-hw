use std::{fs::File, io::Read};
fn main() {
    let mut file = File::open("input.txt").expect("Failed to open file");
    let mut buffer = Vec::new();
    let mut checksum: u16 = 0;
    file.read_to_end(&mut buffer).expect("Failed to read file");
    let now = std::time::Instant::now();
    buffer
        .iter()
        .for_each(|&byte| checksum = (checksum + byte as u16) % 255);
    println!("Time taken: {:?}", now.elapsed());
    println!("Checksum: {:x}", checksum);
}
