use std::net::{SocketAddr, TcpListener, TcpStream};
use std::io;
use std::io::{Read, Write, BufReader, BufRead};
use std::{thread, time};

fn handle_connection(mut stream: TcpStream) -> io::Result<()> {
                    let mut buf = [0; 1];
                    let mut reader = BufReader::new(stream.try_clone()?);
                    stream.peek(&mut buf)?;
                    match buf[0] {
                        4 => println!("SOCKS version: {:?}", buf),
                        5 => println!("SOCKS version: {:?}", buf),
                        _ => {
                            let mut lines = String::new();
                            reader.read_line(&mut lines)?;
                            println!("Non-socks response: {}", lines);
                            return Ok(());
                        }
                    }
                    stream.read_exact(&mut buf)?;
                    stream.write(b"Hello World\r\n")?;
                    /*
                    let mut buf = [0; 128];
                    stream.read(&mut buf)?;
                    println!("{:?} read {:?}", stream, buf);
                    */
                    Ok(())
}

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
            Ok((stream, sockaddr)) => {
                println!("new client: {:?}", sockaddr);
                thread::spawn(move || {
                    match handle_connection(stream) {
                        Ok(_) => {},
                        Err(e) => println!("Error: {}", e)
                    }
                });
            }
        };
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
