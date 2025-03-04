use std::{io::stdout, os::fd::AsRawFd};

use nix::unistd::{close, fork};

fn main() {
    // Question 7: Closing stdout in a child.
    // The After Close println will not be executed.
    match unsafe { fork() } {
        Err(_) => println!("Could not fork"),
        Ok(nix::unistd::ForkResult::Parent { child }) => {
            println!("Parent");
        }
        Ok(nix::unistd::ForkResult::Child) => {
            println!("Before close");
            close(stdout().as_raw_fd()).unwrap();
            println!("After close");
        }
    }
}
