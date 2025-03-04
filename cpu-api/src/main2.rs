use std::os::fd::BorrowedFd;

use nix::{
    fcntl::{OFlag, open},
    sys::stat::Mode,
    unistd::{fork, write},
};

fn main() {
    // Question 2: Concurrent writes to a file descriptor.
    // Writes will interleave as the cpu context switches between the child and parent process.
    let fd = open("test.txt", OFlag::O_CREAT | OFlag::O_WRONLY, Mode::S_IRWXU).unwrap();
    let borrowed_fd = unsafe { BorrowedFd::borrow_raw(fd) };
    match unsafe { fork() } {
        Err(_) => println!("Could not fork"),
        Ok(nix::unistd::ForkResult::Parent { child }) => {
            println!("Writing from parent");
            for i in 0..1000 {
                write(borrowed_fd, format!("PARENT {}\n", i).as_bytes()).unwrap();
            }
        }
        Ok(nix::unistd::ForkResult::Child) => {
            println!("Writing from child");
            for i in 0..1000 {
                write(borrowed_fd, format!("CHILD {}\n", i).as_bytes()).unwrap();
            }
        }
    }
}
