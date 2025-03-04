use std::{ffi::CString, str::FromStr};

use nix::{
    sys::wait::wait,
    unistd::{ForkResult, execvp, fork, getpid},
};

fn main() {
    println!("Hello, world (pid: {})", getpid());

    match unsafe { fork() } {
        Err(_) => panic!("fork failed"),
        Ok(ForkResult::Child) => {
            println!("hello, I am child (pid: {})", getpid());
            let args = ["wc", "Cargo.toml"];
            let c_args = args.map(|a| CString::from_str(a).unwrap());
            execvp(&c_args[0], &c_args).unwrap();
            println!("This shouldn't print out");
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
