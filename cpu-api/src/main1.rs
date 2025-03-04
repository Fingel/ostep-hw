use nix::{sys::wait::wait, unistd::fork};

fn main() {
    // Question 1: Both processes have their own copy of x.
    let mut x = 0;
    match unsafe { fork() } {
        Err(_) => println!("Could not fork"),
        Ok(nix::unistd::ForkResult::Parent { child }) => {
            wait().unwrap();
            x = 2;
            println!("Parent process. x: {}", x);
        }
        Ok(nix::unistd::ForkResult::Child) => {
            println!("Child process. x: {}", x);
            x = 5;
        }
    }
    println!("Hello, world! x: {}", x);
}
