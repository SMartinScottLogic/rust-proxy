use std::net::{SocketAddr, TcpListener, TcpStream};
use std::io;
use std::io::{Read, Write};
use std::{thread, time};

fn server() -> io::Result<()> {
    let addrs = [
        SocketAddr::from(([0, 0, 0, 0, 0, 0, 0, 0], 0)),
        SocketAddr::from(([127, 0, 0, 1], 0)),
    ];
    let listener = TcpListener::bind(&addrs[..])?;
    println!("listening on {:?}", listener.local_addr()?);
    loop {
        match listener.accept() {
            Err(e) => println!("incoming error: {:?}", e),
            Ok((mut stream, sockaddr)) => {
                println!("new client: {:?}", sockaddr);
                thread::spawn(move || {
                    thread::sleep(time::Duration::from_secs(10));
                    stream.write(b"Hello World\r\n").unwrap();
                    let mut buf = [128; 0];
                    stream.read(&mut buf).unwrap();
                    println!("{:?} read {:?}", stream, buf);
                });
            }
        };
    }
    Ok(())
}

fn main() {
    match server() {
        Ok(s) => s,
        Err(e) => {println!("error: {:?}", e); return;}
    };
    println!("Hello, world!");
}
