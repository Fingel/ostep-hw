use std::os::unix::io::AsRawFd;
use std::{ffi::CString, str::FromStr};

use nix::{
    fcntl::{OFlag, open},
    sys::stat::Mode,
    sys::wait::wait,
    unistd::{ForkResult, close, execvp, fork},
};

fn main() {
    match unsafe { fork() } {
        Err(_) => panic!("fork failed"),
        Ok(ForkResult::Child) => {
            // try redirecting stdout to a file
            close(std::io::stdout().as_raw_fd()).unwrap();
            open(
                "./main4.output",
                OFlag::O_CREAT | OFlag::O_WRONLY,
                Mode::S_IRWXU,
            )
            .unwrap();
            let args = ["wc", "Cargo.toml"];
            let c_args = args.map(|a| CString::from_str(a).unwrap());
            execvp(&c_args[0], &c_args).unwrap();
        }
        Ok(ForkResult::Parent { child }) => {
            let wc = wait().unwrap();
            assert!(wc.pid().unwrap() == child);
        }
    }
}
