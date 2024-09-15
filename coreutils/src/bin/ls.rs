use std::path::PathBuf;

use clap::Parser;
use eyre::{Context, Result};

#[derive(Parser)]
struct Cmd {
    directory: PathBuf,
}

fn main() -> Result<()> {
    let args = Cmd::parse();

    let entries = std::fs::read_dir(&args.directory)
        .wrap_err_with(|| format!("opening {}", args.directory.display()))?;

    for entry in entries {
        let entry =
            entry.wrap_err_with(|| format!("reading entry from {}", args.directory.display()))?;

        println!("{}", entry.file_name().to_string_lossy());
    }

    Ok(())
}
