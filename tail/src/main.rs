use nix::fcntl::{OFlag, open};
use nix::sys::stat::Mode;
use nix::unistd::{lseek, read};
use std::{env::args, path::Path};

fn main() {
    const CHUNK_SIZE: usize = 64;
    let n: u32 = args()
        .nth(1)
        .expect("Usage: tail <number> <file>")
        .parse()
        .unwrap();
    let p = args().nth(2).expect("Usage: tail <number> <file>");
    let path = Path::new(&p);
    let fd = open(path, OFlag::O_RDONLY, Mode::S_IRUSR).unwrap();
    let mut output: Vec<u8> = Vec::new();
    let mut offset: i64 = -(CHUNK_SIZE as i64);
    let mut found_newlines = 0;
    let mut buf: [u8; CHUNK_SIZE] = [0; CHUNK_SIZE];
    while found_newlines <= n {
        match lseek(fd, offset, nix::unistd::Whence::SeekEnd) {
            Ok(_) => {}
            Err(_) => break,
        }
        read(fd, &mut buf).unwrap();
        found_newlines += buf.iter().filter(|&b| *b == b'\n').count() as u32;
        offset -= CHUNK_SIZE as i64;
        // Since we're reading backward, we need to prepend new data
        // Reverse the current buffer to maintain the correct order when prepending
        output = buf.into_iter().chain(output).collect();
        // output.extend(&buf);
    }
    // Get the index of the first newline we are interested in.
    let index = output
        .iter()
        .enumerate()
        .filter(|(_, b)| **b == b'\n')
        .nth((found_newlines - n - 1) as usize)
        .map(|(i, _)| i)
        .unwrap_or(0);
    println!("{}", String::from_utf8_lossy(&output[index..]));
}
