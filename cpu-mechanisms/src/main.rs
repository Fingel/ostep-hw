use std::{
    os::fd::{AsFd, AsRawFd},
    process::exit,
    time::Instant,
};

use nix::{
    fcntl::{OFlag, open},
    sched::{CpuSet, sched_setaffinity},
    sys::stat::Mode,
    unistd::{ForkResult, Pid, fork, pipe, read, write},
};

/// Meaure the cost of a system call.
fn measure_system_call_cost(iterations: u32) {
    let x = open("/dev/zero", OFlag::O_RDONLY, Mode::S_IRWXU).unwrap();
    let mut buffer = [0u8; 0];
    let now = Instant::now();
    for _ in 0..iterations {
        let _ = read(x, &mut buffer).unwrap();
    }
    println!("Average system call time: {:?}", now.elapsed() / iterations);
}

fn measure_context_switch(iterations: u32) {
    let mut cpu_set = CpuSet::new();
    cpu_set.set(0).unwrap();

    // Two pipes that will pass a single byte around and around and around.
    let pipe_1 = pipe().unwrap();
    let pipe_1_r = pipe_1.0.as_fd();
    let pipe_1_w = pipe_1.1.as_fd();
    let pipe_2 = pipe().unwrap();
    let pipe_2_r = pipe_2.0.as_fd();
    let pipe_2_w = pipe_2.1.as_fd();

    // Drop the byte in
    write(pipe_1_w, &[0]).unwrap();

    let now = Instant::now();
    match unsafe { fork() } {
        Ok(ForkResult::Parent { child }) => {
            // Not sure if this is doing anything, but supposedly will set the
            // CPU affinity for both process to CPU 0 so we can more accurately measure
            // the time it takes to switch between two threads.
            sched_setaffinity(Pid::from_raw(0), &cpu_set).unwrap();
            sched_setaffinity(Pid::from_raw(child.as_raw()), &cpu_set).unwrap();

            let mut buf = [0u8; 1];
            for _ in 0..iterations {
                read(pipe_1_r.as_raw_fd(), &mut buf).unwrap();
                write(pipe_2_w, &buf).unwrap();
            }
        }
        Ok(ForkResult::Child) => {
            let mut buf = [0u8; 1];
            for _ in 0..iterations {
                read(pipe_2_r.as_raw_fd(), &mut buf).unwrap();
                write(pipe_1_w, &buf).unwrap();
            }
            exit(0);
        }
        Err(err) => panic!("fork failed: {}", err),
    }
    println!("Context switch takes {:?}", now.elapsed() / iterations);
}
fn main() {
    // Homework 1: Measure the time it takes to make a system call.
    // On my system doing a 0 byte read of /dev/zero takes about 100 nanoseconds.
    measure_system_call_cost(1_000_000);

    // Homework 2: Measure the time it takes to switch between two threads.
    // On my sustem doing a context switch takes about 2.3 microseconds.
    measure_context_switch(1_000_000);
}
