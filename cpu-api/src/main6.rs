use nix::{
    sys::wait::{wait, waitpid},
    unistd::fork,
};

fn main() {
    // Question 6: Using waitpid.
    // Waitpid takes a pid as an argument, which is useful if there are multiple children. It also
    // takes options that control how the wait is performed based on the child's state.
    match unsafe { fork() } {
        Err(_) => println!("Could not fork"),
        Ok(nix::unistd::ForkResult::Parent { child }) => {
            let waitstatus = waitpid(child, None).unwrap();
            println!("Parent");
        }
        Ok(nix::unistd::ForkResult::Child) => {
            println!("Child");
        }
    }
}
