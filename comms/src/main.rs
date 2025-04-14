use std::io;
use std::io::ErrorKind;
use std::net::UdpSocket;
use std::time::Duration;
use std::{env::args, net::Ipv4Addr};

static MAX_PACKET_SIZE: usize = 1022;

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
                    let buf = &mut buf[..amt];
                    println!(
                        "Received {} bytes from {} id {} chunk {}",
                        amt, addr, buf[0], buf[1]
                    );
                    socket.send_to("ack".as_bytes(), addr).unwrap();
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
        if let Err(e) = socket.set_read_timeout(Some(Duration::new(3, 0))) {
            eprintln!("Error setting read timeout: {}", e);
        }
        println!(
            "Client running on {}:{}",
            self.remote_addr,
            socket.local_addr().unwrap().port()
        );
        let msg = [0u8; MAX_PACKET_SIZE * 3];
        if msg.len() > MAX_PACKET_SIZE {
            msg.chunks(MAX_PACKET_SIZE)
                .enumerate()
                .for_each(|(i, chunk)| {
                    println!("Chunk {} sent", i);
                    self.send(&socket, chunk, 1, i as u8).unwrap();
                });
        }
    }

    fn send(&self, socket: &UdpSocket, data: &[u8], id: u8, total: u8) -> Result<(), io::Error> {
        loop {
            println!("Sending {} bytes of data...", data.len());
            let packet = [id, total]
                .iter()
                .cloned()
                .chain(data.iter().cloned())
                .collect::<Vec<u8>>();
            match self.send_with_ack(socket, &packet) {
                Ok(_) => {
                    println!("Sent data");
                    return Ok(());
                }
                Err(e) => match e.kind() {
                    ErrorKind::TimedOut | ErrorKind::WouldBlock => {
                        println!("Timed out");
                        continue;
                    }
                    _ => {
                        println!("Other error");
                        return Err(e);
                    }
                },
            }
        }
    }

    fn send_with_ack(&self, socket: &UdpSocket, data: &[u8]) -> Result<(), io::Error> {
        socket.send_to(data, (self.remote_addr, 8777))?;
        let mut buf = [0; 1024];
        match socket.recv_from(&mut buf) {
            Ok((amt, _)) => {
                let buf = &mut buf[..amt];
                if String::from_utf8_lossy(buf).starts_with("ack") {
                    println!("Received ack");
                    Ok(())
                } else {
                    Err(io::Error::new(io::ErrorKind::Other, "Unexpected response"))
                }
            }
            Err(e) => Err(e),
        }
    }
}
