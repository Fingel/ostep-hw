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
    let mut found_newlines = 0;
    let fd = open(path, OFlag::O_RDONLY, Mode::S_IRUSR).unwrap();
    let mut offset = -1;
    let mut buf: [u8; 4096] = [0; 4096];
    while found_newlines < n + 1 {
        lseek(fd, offset, nix::unistd::Whence::SeekEnd).unwrap();
        read(fd, &mut buf).unwrap();
        found_newlines = buf.iter().filter(|&b| *b == b'\n').count() as u32;
        offset -= 1;
    }
    println!("{}", String::from_utf8_lossy(&buf));
}
