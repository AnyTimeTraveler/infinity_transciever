use std::env::args;
use std::io::{Read, Write};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream};
use std::thread;

const SIZE: usize = 1024 * 10;
const START_PORT: u16 = 5000;
const END_PORT: u16 = 5100;

fn main() {
    let args_vec: Vec<String> = args().collect();
    let mut joins = Vec::new();

    if args_vec.len() <= 1 {
        for port in START_PORT..=END_PORT {
            let port = port;
            joins.push(thread::spawn(move || {
                let listener = TcpListener::bind(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), port));
                match listener {
                    Ok(listener) => {
                        println!("Listening on port {}!", port);
                        loop {
                            let result = listener.accept();
                            if let Ok((mut stream, addr)) = result {
                                println!("{}:{} connected!", addr.ip(), addr.port());
                                let buf = [0u8; SIZE];
                                let mut counter = 0u64;
                                while stream.write_all(&buf).is_ok() {
                                    counter += 1;
                                }
                                println!("Done sending data to: {}:{}", addr.ip(), addr.port());
                                println!("Sent: {} MB", counter / SIZE as u64);
                            } else {
                                println!("Error while receiving new connection!");
                            }
                        }
                    }
                    Err(e) => println!("Error connecting: {}", e),
                }
            }));
        }
    } else {
        let mut args = args_vec.iter();
        args.next().unwrap();
        let ip: Ipv4Addr = args.next().unwrap().parse().expect("Error parsing IP!");
        while let Some(port) = args.next().cloned() {
            joins.push(thread::spawn(move || {
                let stream = TcpStream::connect(SocketAddr::new(IpAddr::from(ip), port.parse().unwrap()));
                match stream {
                    Ok(mut stream) => {
                        println!("Connected to {}:{}!", ip, port);
                        let mut buf = [0u8; SIZE];
                        let mut counter = 0;
                        while let Ok(bytes_read) = stream.read(&mut buf) {
                            if bytes_read == 0 {
                                break;
                            }
                            counter += bytes_read;
                        }
                        println!("Done receiving data from: {}:{}", ip, port);
                        println!("Received: {} MB", counter / (1024 * 1024));
                    }
                    Err(e) => println!("Error connecting: {}", e),
                }
            }));
        }
    }

    for join in joins {
        join.join().expect("Error waiting for thread!");
    }
}
