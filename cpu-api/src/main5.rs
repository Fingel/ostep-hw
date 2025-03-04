use nix::{sys::wait::wait, unistd::fork};

fn main() {
    // Question 5: Using wait.
    // Wait returns a wait status. In this nix crate, it's an enum of possible states of the child process.
    // Calling wait() in the child panics with the error ECHILD which makes sense because it has no children.
    match unsafe { fork() } {
        Err(_) => println!("Could not fork"),
        Ok(nix::unistd::ForkResult::Parent { child }) => {
            let waitstatus = wait().unwrap();
            println!("Parent");
        }
        Ok(nix::unistd::ForkResult::Child) => {
            println!("Child");
        }
    }
}
