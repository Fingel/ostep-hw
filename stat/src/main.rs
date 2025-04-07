use std::{env::args, path::Path};

use nix::sys::stat;

fn main() {
    if args().len() < 2 {
        println!(
            "Usage: {} <file>",
            args().next().unwrap_or("stat".to_string())
        );
        return;
    }
    let p = args().next().unwrap().to_string();
    let f = Path::new(&p);
    let stats = stat::stat(f).unwrap();
    println!("File Size: {} bytes", stats.st_size);
    println!("Blocks: {} 512 byte blocks", stats.st_blocks);
    println!("References: {}", stats.st_nlink);
    println!("Device ID: {}", stats.st_dev);
    println!("Inode Number: {}", stats.st_ino);
}
