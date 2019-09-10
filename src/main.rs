use std::net::{SocketAddr, TcpListener, TcpStream};
use std::io;

fn server() -> io::Result<()> {
    let addrs = [
        SocketAddr::from(([0, 0, 0, 0, 0, 0, 0, 0], 0)),
        SocketAddr::from(([127, 0, 0, 1], 0)),
    ];
    let listener = TcpListener::bind(&addrs[..])?;
    println!("listening on {:?}", listener.local_addr()?);
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("new client: {:?}", stream);
            }
            Err(e) => println!("incoming error: {:?}", e)
        }
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
