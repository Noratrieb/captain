use eyre::Result;
use tracing::{error, info, warn};

fn main() {
    tracing_subscriber::fmt().init();
    info!("Booting up system");

    if let Err(err) = run() {
        error!(?err, "Failed to boot system");
    }

    // uh.. i dont think we should exit?
}

fn run() -> Result<()> {
    if let Err(err) = rustix::thread::set_no_new_privs(true) {
        warn!(?err, "Failed to set PR_SET_NO_NEW_PRIVS");
    }

    std::process::Command::new("/bin/cog")
        .env("PATH", "/bin")
        .spawn()?
        .wait()?;

    loop {}
}
