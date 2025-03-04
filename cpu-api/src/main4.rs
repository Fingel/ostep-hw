use std::{
    ffi::{CStr, CString},
    os::fd::BorrowedFd,
    str::FromStr,
};

use nix::{
    fcntl::{OFlag, open},
    libc::execl,
    sched::sched_yield,
    sys::stat::Mode,
    unistd::{execv, execve, execvp, execvpe, fork, write},
};

fn main() {
    // Question 4: Call ls a bunch of times
    // Execv replaces the current process with a new process image, so the println never gets executed.
    // Execve is the same as execv, but takes in env variables.
    // Execvp is the same as execv, but takes the shell's PATH into account, so one can just call `ls`.
    // Execvpe is a combo of execve and execvp
    // Execveat allows one to specify a directory to search for the executable
    // Fexecve is the same as execve, but takes in file descriptor as the first argument instead of a path.
    // There are so many variants because there are so many different situations for executing programs.
    match unsafe { fork() } {
        Err(_) => println!("Could not fork"),
        Ok(nix::unistd::ForkResult::Parent { child }) => {
            println!("Parent");
        }
        Ok(nix::unistd::ForkResult::Child) => {
            let args = ["ls"];
            let c_args = args.map(|a| CString::from_str(a).unwrap());
            let c_env: [CString; 0] = [];
            println!("Child");
            execvpe(&c_args[0], &c_args, &c_env).unwrap();
        }
    }
}
