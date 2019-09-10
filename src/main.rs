use std::net::{SocketAddr, TcpListener, TcpStream};
use std::io;
use std::thread;
use std::io::{Read, Write};

fn server() -> io::Result<()> {
    let addrs = [
        SocketAddr::from(([0, 0, 0, 0, 0, 0, 0, 0], 0)),
        SocketAddr::from(([127, 0, 0, 1], 0)),
    ];
    let listener = TcpListener::bind(&addrs[..])?;
    println!("listening on {:?}", listener.local_addr()?);
    for stream in listener.incoming() {
        thread::spawn(|| {
        match stream {
            Ok(mut stream) => {
                println!("new client: {:?}", stream);
                stream.write(b"Hello World\r\n").unwrap();
                let mut buf = [128; 0];
                stream.read(&mut buf).unwrap();
                println!("{:?} read {:?}", stream, buf);
            }
            Err(e) => println!("incoming error: {:?}", e)
        }
        });
    };
    Ok(())
}

fn main() {
    match server() {
        Ok(s) => s,
        Err(e) => {println!("error: {:?}", e); return;}
    };
    println!("Hello, world!");
}
