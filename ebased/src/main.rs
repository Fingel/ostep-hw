use nix::errno::Errno;
use nix::fcntl::{OFlag, open};
use nix::sys::aio::{Aio, AioRead, aio_suspend};
use nix::sys::select::FdSet;
use nix::sys::signal::SigevNotify;
use nix::sys::socket::{
    self, AddressFamily, Backlog, SockFlag, SockType, SockaddrIn, bind, listen, setsockopt, socket,
    sockopt::{ReuseAddr, ReusePort},
};
use nix::sys::stat::Mode;
use nix::unistd::{read, write};
use std::os::fd::{AsFd, AsRawFd, BorrowedFd, FromRawFd, OwnedFd};
use std::path::Path;
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

                                // Read the contents of the socket
                                let mut buffer = [0; 1024];
                                let bytes_read = read(client_fd.as_raw_fd(), &mut buffer).unwrap();
                                let filename = String::from_utf8_lossy(&buffer[..bytes_read]);
                                println!("Received filename: {}", filename);
                                let p_what = filename.to_string();
                                let path = Path::new(&p_what);
                                println!("{}", path.exists());

                                let file_fd = open(path, OFlag::O_RDONLY, Mode::empty()).unwrap();
                                let mut file_content = [0; 1024];
                                let length: usize;
                                {
                                    let file_fd_borrowed =
                                        unsafe { BorrowedFd::borrow_raw(file_fd) };
                                    let mut aior = Box::pin(AioRead::new(
                                        file_fd_borrowed,
                                        0,
                                        &mut file_content,
                                        0,
                                        SigevNotify::SigevNone,
                                    ));
                                    aior.as_mut().submit().unwrap();
                                    aio_suspend(&[&*aior], None).unwrap();
                                    length = aior.as_mut().aio_return().unwrap();
                                }

                                let file_content = String::from_utf8_lossy(&file_content[..length]);
                                write(client_fd, file_content.as_bytes()).unwrap();
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
