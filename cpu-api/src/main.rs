use std::{
    io::stdin,
    io::stdout,
    os::fd::{AsFd, AsRawFd},
};

use nix::{
    sys::wait::wait,
    unistd::{dup2, fork, pipe, read},
};

fn main() {
    // Question 8: Pipes
    // This one is a doozy. We don't need the explicit close() calls like in C because the borrowed
    // fds are automatically closed when dropped. At least that's what I think is happening.
    let (read_end_owned, write_end_owned) = pipe().unwrap();
    let write_end = write_end_owned.as_fd();
    let read_end = read_end_owned.as_fd();
    match unsafe { fork() } {
        Err(_) => println!("Could not fork"),
        Ok(nix::unistd::ForkResult::Parent { child: _ }) => {
            println!("Parent");
            wait().unwrap();
            match unsafe { fork() } {
                Err(_) => println!("Could not fork"),
                Ok(nix::unistd::ForkResult::Parent { child: _ }) => {
                    println!("Parent");
                    wait().unwrap();
                }
                Ok(nix::unistd::ForkResult::Child) => {
                    println!("Hello, I am Child 2 ");
                    dup2(read_end.as_raw_fd(), stdin().as_raw_fd()).unwrap();
                    let mut buffer = [0; 1024];
                    let n = read(stdin().as_raw_fd(), &mut buffer).unwrap();
                    println!(
                        "Read {} bytes from sibling: {}",
                        n,
                        String::from_utf8_lossy(&buffer)
                    );
                }
            }
        }
        Ok(nix::unistd::ForkResult::Child) => {
            println!("Hello, I am child 1");
            dup2(write_end.as_raw_fd(), stdout().as_raw_fd()).unwrap();
            println!("Hello, I am child 1 writing to the pipe");
        }
    }
}
