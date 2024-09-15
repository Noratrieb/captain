use clap::Parser;
use eyre::Result;

#[derive(Parser)]
struct Cmd {
    #[command(subcommand)]
    cmd: Subcommand,
}

#[derive(clap::Subcommand)]
enum Subcommand {
    /// Get the current user's uid
    Id,
}

fn main() -> Result<()> {
    let args = Cmd::parse();

    match args.cmd {
        Subcommand::Id => {
            println!("{}", rustix::process::getuid().as_raw());
        }
    }

    Ok(())
}
