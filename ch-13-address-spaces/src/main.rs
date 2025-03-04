use std::env;
use std::mem::size_of;
use std::thread::sleep;
use std::time::Duration;
fn main() {
    let args: Vec<String> = env::args().collect();
    let megabytes = &args[1].parse::<usize>().unwrap();
    let buf = vec![255u8; 1024 * 1024 * megabytes];
    dbg!(megabytes);
    dbg!(size_of::<u8>() * buf.capacity());
    loop {
        sleep(Duration::from_secs(1));
    }
}
