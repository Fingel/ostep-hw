use std::net::UdpSocket;
use std::{env::args, net::Ipv4Addr};

fn main() {
    let mode = args().nth(1).unwrap_or("client".to_string());
    if mode == "server" {
        println!("Starting server...");
        let server = Server::new(Ipv4Addr::new(127, 0, 0, 1), 8777);
        server.run();
    } else if mode == "client" {
        println!("Starting client...");
        let client = Client::new();
        client.run();
    } else {
        println!("Usage: {} <server|client>", mode);
    }
}

#[derive(Debug)]
struct Server {
    addr: Ipv4Addr,
    port: u16,
}

impl Server {
    fn new(addr: Ipv4Addr, port: u16) -> Self {
        Server { addr, port }
    }
    fn run(&self) {
        let socket = UdpSocket::bind((self.addr, self.port)).unwrap();
        println!("Server running on {}:{}", self.addr, self.port);
        loop {
            let mut buf = [0; 1024];
            match socket.recv_from(&mut buf) {
                Ok((amt, addr)) => {
                    println!("Received {} bytes from {}", amt, addr);
                    let buf = &mut buf[..amt];
                    buf.reverse();
                    socket.send_to(buf, addr).unwrap();
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                }
            }
        }
    }
}

struct Client {
    remote_addr: Ipv4Addr,
}

impl Client {
    fn new() -> Self {
        Client {
            remote_addr: Ipv4Addr::new(127, 0, 0, 1),
        }
    }
    fn run(&self) {
        let socket = UdpSocket::bind((Ipv4Addr::new(127, 0, 0, 1), 0)).unwrap();
        println!(
            "Client running on {}:{}",
            self.remote_addr,
            socket.local_addr().unwrap().port()
        );
        let buf = "Hello, world!";
        socket
            .send_to(buf.as_bytes(), (self.remote_addr, 8777))
            .unwrap();
    }
}
