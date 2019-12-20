use std::net::{SocketAddr, TcpListener, TcpStream};
use std::io;
use std::io::{Read, Write, BufReader, BufRead, BufWriter};
use std::{thread, time};
use futures::{
        // Extension trait for futures 0.1 futures, adding the `.compat()` method
        // which allows us to use `.await` on 0.1 futures.
        compat::Future01CompatExt,
        // Extension traits providing additional methods on futures.
        // `FutureExt` adds methods that work for all futures, whereas
        // `TryFutureExt` adds methods to futures that return `Result` types.
        future::{FutureExt, TryFutureExt},
        executor::block_on,
};

fn handle_socks4(mut reader: BufReader<TcpStream>, mut writer: BufWriter<TcpStream>) -> io::Result<()> {
    Ok(())
}

fn handle_socks5(mut reader: BufReader<TcpStream>, mut writer: BufWriter<TcpStream>) -> io::Result<()> {
    Ok(())
}

fn handle_http(mut reader: BufReader<TcpStream>, mut writer: BufWriter<TcpStream>) -> io::Result<()> {
    let mut buf = String::new();
    reader.read_line(&mut buf)?;
    let request_components = buf.split(' ').collect::<Vec<_>>();
    println!("bits: {:?}", request_components);
    let method = request_components.get(0).map_or(Err(std::io::Error::new(std::io::ErrorKind::Other, "invalid request")), |v| Ok(v))?;
    println!("  METHOD: {}", method);

    let read = async {
        let delay = time::Duration::from_millis(10);
    loop {
        let mut buf = [0; 1024];
        let size = reader.read(&mut buf)?;
        /*
        if buf.trim().len() == 0 {
            println!("END OF HEADERS");
            //break;
        }
        */
        if size > 0 {
            hexdump(&buf, size);
        }
        thread::sleep(delay);
    }
    Ok::<(), io::Error>(())
    };
    /*
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
    */

    let delay = time::Duration::from_secs(2);
    thread::sleep(delay);
println!("Send response...");
    writer.write_all(b"HTTP/1.1 200 Empty\r\n")?;
    writer.write_all(b"Connection: close\r\n")?;
    writer.write_all(b"\r\n")?;
    writer.write_all(b"Hello World\r\n")?;
    writer.write_all(b"\r\n")?;
    writer.flush()?;
println!("Sent");

    block_on(read)?;
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

fn hexdump(buf: &[u8], size: usize) {
    println!("size: {}", size);
    let mut hex_line = String::new();
    let mut ascii_line = String::new();
    for b in buf {
        if *b > 31 && *b < 127 {
            ascii_line.push(*b as char);
        } else {
            ascii_line.push('.');
        }
        hex_line.push_str(format!("{:02x?} ", *b).as_str());

        if ascii_line.len()==16 {
            println!("{} {}", hex_line, ascii_line);
            ascii_line = String::new();
            hex_line = String::new();
        }
    }
    println!("{} {}", hex_line, ascii_line);
}

fn main() {
    match server() {
        Ok(s) => s,
        Err(e) => {println!("error: {:?}", e); return;}
    };
    println!("Hello, world!");
}
