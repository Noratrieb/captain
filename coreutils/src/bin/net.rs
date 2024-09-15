use std::net::{TcpStream, ToSocketAddrs};

use clap::Parser;
use eyre::{Context, Result};

#[derive(Parser)]
struct Cmd {
    #[command(subcommand)]
    cmd: Subcommand,
}

#[derive(clap::Subcommand)]
enum Subcommand {
    /// Performs DNS resolution of a domain name
    Resolve { addr: String },
    Tcp {
        #[command(subcommand)]
        cmd: Tcp,
    },
}

#[derive(clap::Subcommand)]
enum Tcp {
    /// Connect via TCP and use stdin/stdout to communicate
    Connect { addr: String },
}

fn main() -> Result<()> {
    let args = Cmd::parse();

    match args.cmd {
        Subcommand::Resolve { addr } => {
            let addrs = (addr.as_str(), 0)
                .to_socket_addrs()
                .wrap_err_with(|| format!("resolving {addr}"))?;

            for addr in addrs {
                println!("{}", addr.ip());
            }
        }
        Subcommand::Tcp {
            cmd: Tcp::Connect { addr },
        } => {
            let mut stream = TcpStream::connect(&addr).wrap_err_with(|| {
                format!("connecting to {addr}. note: use IP:PORT as the address format")
            })?;
            let mut reader = stream.try_clone().wrap_err("cloning stream")?;

            std::thread::spawn(move || std::io::copy(&mut reader, &mut std::io::stdout().lock()));

            std::io::copy(&mut std::io::stdin().lock(), &mut stream)?;
        }
    }

    Ok(())
}
