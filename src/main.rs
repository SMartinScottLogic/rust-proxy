use futures::{
    // Extension trait for futures 0.1 futures, adding the `.compat()` method
    // which allows us to use `.await` on 0.1 futures.
    compat::Future01CompatExt,
    executor::block_on,
    // Extension traits providing additional methods on futures.
    // `FutureExt` adds methods that work for all futures, whereas
    // `TryFutureExt` adds methods to futures that return `Result` types.
    future::{FutureExt, TryFutureExt},
};
use std::io;
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream, ToSocketAddrs};
use std::str;
use std::{thread, time};
use url::{ParseError, Url};

mod transfer;
use transfer::Transfer;

struct DirectTransfer {
    source: BufReader<TcpStream>,
    target: BufWriter<TcpStream>,
}

impl DirectTransfer {
    fn new(source: BufReader<TcpStream>, target: BufWriter<TcpStream>) -> DirectTransfer {
        DirectTransfer { source, target }
    }
}

impl Transfer for DirectTransfer {
    fn run(&self) {}
}

fn handle_socks4(
    mut reader: BufReader<TcpStream>,
    mut writer: BufWriter<TcpStream>,
) -> io::Result<()> {
    Ok(())
}

fn handle_socks5(
    mut reader: BufReader<TcpStream>,
    mut writer: BufWriter<TcpStream>,
) -> io::Result<()> {
    Ok(())
}

fn handle_http(
    mut reader: BufReader<TcpStream>,
    mut writer: BufWriter<TcpStream>,
) -> io::Result<()> {
    let mut buf = String::new();
    reader.read_line(&mut buf)?;
    let mut request_components = buf.trim().split(' ').collect::<Vec<_>>();
    println!("bits: {:?}", request_components);
    let method = request_components.get(0).map_or(
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "invalid request",
        )),
        |v| Ok(v),
    )?;
    let url = request_components.get(1).map_or(
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "invalid request",
        )),
        |v| Ok(v),
    )?;
    let url = match Url::parse(url) {
        Err(_) => Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "invalid request: ".to_owned() + url,
        )),
        Ok(v) => Ok(v),
    }?;

    println!("  METHOD: {}", method);
    println!("  URL: {:?}", url);
    println!("  PATH: {:?}", url.path());
    println!("  QUERY: {:?}", url.query());
    println!("  HOST: {:?}", url.host_str().unwrap());
    println!("  PORT: {:?}", url.port_or_known_default());
    println!("  ORIGIN: {:?}", url.origin());
    println!("  ORIGIN: {:?}", url.origin().unicode_serialization());
    println!("  HOST: {:?}", url.host_str().unwrap().to_socket_addrs());
    let hostport = format!(
        "{}:{}",
        url.host_str().unwrap(),
        url.port_or_known_default().unwrap_or(80)
    );
    println!("  HOSTPORT: {} {:?}", hostport, hostport.to_socket_addrs());
    let mut outward = TcpStream::connect(hostport)?;
    println!("  OUTWARD: {:?}", outward);
    let path = match url.query() {
        Some(query) => format!("{}?{}", url.path(), query),
        None => url.path().to_string(),
    };
    println!("  PATH: {}", path);
    request_components[1] = &path;
    println!("bits: {:?}", request_components);
    hexdump(request_components.join(" ").as_bytes());
    write!(outward, "{}", request_components.join(" "));
    println!("{}", request_components.join(" "));
    outward.write_all(b"\r\n");
    write!(outward, "Host: {}", url.host_str().unwrap_or(""));
    println!("Host: {}", url.host_str().unwrap_or(""));
    outward.write_all(b"\r\n");
    outward.write_all(b"\r\n");
    outward.flush();
    let outward_read = async {
        loop {
            let mut buf = [0; 1000];
            let size = outward.read(&mut buf)?;
            if size > 0 {
                //hexdump(&buf[..size]);
                let mut o = String::new();
                for b in &buf[..size] {
                    o.push(*b as char);
                }
                println!("{}", o);
            }
        }
        Ok::<(), io::Error>(())
    };

    block_on(outward_read)?;

    let read = async {
        let delay = time::Duration::from_millis(10);
        let mut all = Vec::new();
        loop {
            let mut buf = [0; 11];
            let size = reader.read(&mut buf)?;
            /*
            if buf.trim().len() == 0 {
                println!("END OF HEADERS");
                //break;
            }
            */
            if size > 0 {
                hexdump(&buf[..size]);
                all.extend_from_slice(&buf[..size]);
                hexdump(&all);
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

    let outward_reader = BufReader::new(outward.try_clone()?);
    let outward_writer = BufWriter::new(outward.try_clone()?);

    let transfer_out = DirectTransfer::new(reader, outward_writer);
    let transfer_in = DirectTransfer::new(outward_reader, writer);

    transfer_out.run();
    transfer_in.run();
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
        _ => handle_http(reader, writer),
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
                thread::spawn(move || match handle_connection(stream) {
                    Ok(_) => {}
                    Err(e) => println!("Error: {}", e),
                });
            }
        };
    }
    Ok(())
}

fn hexdump(buf: &[u8]) {
    println!("size: {}", buf.len());
    let mut hex_line = String::new();
    let mut ascii_line = String::new();
    for b in buf {
        if *b > 31 && *b < 127 {
            ascii_line.push(*b as char);
        } else {
            ascii_line.push('.');
        }
        hex_line.push_str(format!("{:02x?} ", *b).as_str());

        if ascii_line.len() == 32 {
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
        Err(e) => {
            println!("error: {:?}", e);
            return;
        }
    };
    println!("Hello, world!");
}
