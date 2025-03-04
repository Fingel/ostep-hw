use std::{thread::sleep, time::Duration};

use nix::{
    sys::wait::wait,
    unistd::{ForkResult, fork, getpid},
};

fn main() {
    println!("Hello, world (pid: {})", getpid());

    match unsafe { fork() } {
        Err(_) => panic!("fork failed"),
        Ok(ForkResult::Child) => {
            println!("hello, I am child (pid: {})", getpid());
            sleep(Duration::from_secs(1));
        }
        Ok(ForkResult::Parent { child }) => {
            let wc = wait().unwrap();
            println!(
                "hello, I am parent of {} (wc: {}) (pid: {})",
                child,
                wc.pid().unwrap(),
                getpid()
            );
        }
    }
}
