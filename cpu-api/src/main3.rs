use std::os::fd::BorrowedFd;

use nix::{
    fcntl::{OFlag, open},
    sched::sched_yield,
    sys::stat::Mode,
    unistd::{fork, write},
};

fn main() {
    // Question 3: Print in child before Parent
    // The only way to get the child to execute first is to use wait() or /proc/sys/kernel/sched_child_runs_first
    // All other methods seem jank.
    match unsafe { fork() } {
        Err(_) => println!("Could not fork"),
        Ok(nix::unistd::ForkResult::Parent { child }) => {
            sched_yield().unwrap();
            println!("Goodbye");
        }
        Ok(nix::unistd::ForkResult::Child) => {
            println!("Hello");
        }
    }
}
