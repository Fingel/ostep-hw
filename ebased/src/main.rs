use nix::sys::select::FdSet;
use nix::sys::socket::{
    self, AddressFamily, Backlog, SockFlag, SockType, SockaddrIn, bind, listen, setsockopt, socket,
    sockopt::{ReuseAddr, ReusePort},
};
use nix::unistd::write;
use std::os::fd::{AsFd, AsRawFd, BorrowedFd, FromRawFd, OwnedFd, RawFd};
use std::time::SystemTime;
fn main() -> std::io::Result<()> {
    let num_sockets = 10;
    let server_addr = SockaddrIn::new(127, 0, 0, 1, 8080);
    let mut sockets = Vec::with_capacity(num_sockets);

    for _ in 0..num_sockets {
        let sock_fd = socket(
            AddressFamily::Inet,
            SockType::Stream,
            SockFlag::empty(),
            None,
        )
        .unwrap();

        setsockopt(&sock_fd, ReuseAddr, &true)?;
        setsockopt(&sock_fd, ReusePort, &true)?;
        bind(sock_fd.as_raw_fd(), &server_addr)?;
        listen(&sock_fd, Backlog::MAXALLOWABLE)?;
        sockets.push(sock_fd);
    }

    let mut fd_set = FdSet::new();
    for sock in &sockets {
        fd_set.insert(sock.as_fd());
    }
    let mut counter = 0;
    loop {
        // Create a copy of fd_set for select() as it modifies the set
        let mut read_set = fd_set;

        // Call select with timeout
        match nix::sys::select::select(
            None,
            &mut read_set,
            None,
            None,
            &mut nix::sys::time::TimeVal::new(5, 0),
        ) {
            Ok(num_ready) if num_ready > 0 => {
                dbg!(num_ready);
                // Process ready file descriptors
                for sock_fd in &sockets {
                    // Check if this socket is ready
                    if read_set.contains(sock_fd.as_fd()) {
                        // Handle the connection
                        match socket::accept(sock_fd.as_raw_fd()) {
                            Ok(raw_client_fd) => {
                                let client_fd = unsafe { OwnedFd::from_raw_fd(raw_client_fd) };
                                // Handle client connection
                                println!("Accepted connection on socket {}", sock_fd.as_raw_fd());

                                // For simplicity, we'll close it right away
                                // In a real app, you'd spawn a thread or handle async
                                let now = SystemTime::now();
                                let msg = format!("Hello, client! It's {:?} o'clock\n", now);
                                write(client_fd, msg.as_bytes()).unwrap();
                                // let _ = nix::unistd::close(client_fd);
                            }
                            Err(e) => eprintln!("Error accepting connection: {}", e),
                        }
                    }
                }
            }
            Ok(_) => {
                // Timeout or no events
                println!("Timeout or no events");
            }
            Err(e) => {
                eprintln!("Error in select: {}", e);
            }
        }
        counter += 1;
        println!("Total loops: {}", counter);
    }
}
