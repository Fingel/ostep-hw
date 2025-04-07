use nix::fcntl::{OFlag, open};
use nix::libc::printf;
use nix::sys::stat::Mode;
use nix::unistd::{lseek, read};
use std::{env::args, path::Path};

fn main() {
    let n: u32 = args()
        .nth(1)
        .expect("Usage: tail <number> <file>")
        .parse()
        .unwrap();
    let p = args().nth(2).expect("Usage: tail <number> <file>");
    let path = Path::new(&p);
    let fd = open(path, OFlag::O_RDONLY, Mode::S_IRUSR).unwrap();
    let mut offset = -64;
    let mut buf: [u8; 4096] = [0; 4096];
    let mut found_newlines = 0;
    while found_newlines <= n {
        lseek(fd, offset, nix::unistd::Whence::SeekEnd).unwrap();
        read(fd, &mut buf).unwrap();
        found_newlines = buf.iter().filter(|&b| *b == b'\n').count() as u32;
        offset -= 64;
    }
    // Get the index of the first newline we are interested in.
    let index = buf
        .iter()
        .enumerate()
        .filter(|(_, b)| **b == b'\n')
        .nth((found_newlines - n - 1) as usize)
        .map(|(i, _)| i)
        .unwrap_or(0);
    println!("{}", String::from_utf8_lossy(&buf[index..]));
}
