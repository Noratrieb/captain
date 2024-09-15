use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

use eyre::{Context, Result};

fn main() -> Result<()> {
    let addr = std::env::var("BIND_ADDR")
        .ok()
        .or(std::env::args().nth(1))
        .unwrap_or("127.0.0.1:1000".to_owned());
    println!("Listening on address {addr}");
    let stream = TcpListener::bind(&addr).wrap_err_with(|| format!("binding socket on {addr}"))?;

    loop {
        let next = stream.accept()?;
        std::thread::spawn(move || {
            if let Err(err) = echo(next.0) {
                eprintln!("{err:?}");
            }
        });
    }
}

fn echo(mut stream: TcpStream) -> Result<()> {
    let mut buf = [0; 1024];
    loop {
        let read = stream.read(&mut buf)?;
        if read == 0 {
            return Ok(());
        }
        stream.write_all(&buf[..read])?;
    }
}
