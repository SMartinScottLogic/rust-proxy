use std::net::{SocketAddr, TcpListener, TcpStream};
use std::io;
use std::io::{Read, Write, BufReader, BufRead, BufWriter};
use std::{thread, time};

fn handle_socks4(mut reader: BufReader<TcpStream>, mut writer: BufWriter<TcpStream>) -> io::Result<()> {
    Ok(())
}

fn handle_socks5(mut reader: BufReader<TcpStream>, mut writer: BufWriter<TcpStream>) -> io::Result<()> {
    Ok(())
}

fn handle_http(mut reader: BufReader<TcpStream>, mut writer: BufWriter<TcpStream>) -> io::Result<()> {
    let mut lines = Vec::new();
    loop {
        let mut buf = String::new();
        reader.read_line(&mut buf)?;
        if buf.trim().len() == 0 {
            break;
        }
        println!("{} {}", buf.len(), buf);
        lines.push(buf);
    }

    //let lines = reader.lines().filter_map(io::Result::ok).collect::<Vec<String>>();

    println!("Non-socks response: {:?}", lines);
    writer.write_fmt(format_args!("HTTP/1.1 200 Empty\r\n"))?;
    writer.write_all(b"Connection: close\r\n")?;
    writer.write_all(b"\r\n")?;
    writer.write_all(b"Hello World\r\n")?;
    writer.write_all(b"\r\n")?;
    Ok(())
}

fn handle_connection(stream: TcpStream) -> io::Result<()> {
                    let mut buf = [0; 1];
                    let reader = BufReader::new(stream.try_clone()?);
                    let writer = BufWriter::new(stream.try_clone()?);
                    stream.peek(&mut buf)?;
                    match buf[0] {
                        4 => handle_socks4(reader, writer),
                        5 => handle_socks5(reader, writer),
                        _ => handle_http(reader, writer)
                    }?;
                    /*
                    stream.read_exact(&mut buf)?;
                    stream.write(b"Hello World\r\n")?;
                    */
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
