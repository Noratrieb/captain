use std::{
    fs::{DirEntry, File},
    path::Path,
    process::Stdio,
    time::Duration,
};

use eyre::{bail, eyre, Context, Result};
use rustix::{io::Errno, process::WaitOptions};
use serde::Deserialize;
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

    let result = std::thread::Builder::new()
        .name("reaper".to_owned())
        .spawn(|| reaper());
    if let Err(err) = result {
        error!(?err, "Failed to spawn reaper thread");
    }

    let services = read_services(Path::new("/etc/services"));
    match services {
        Ok((services, errors)) => {
            for error in errors {
                error!(?error, "Failed to read service");
            }

            for service in services {
                std::thread::spawn(move || {
                    if let Err(err) = run_service(&service) {
                        error!(name = %service.name, ?err, "Failed to spawn service");
                    }
                });
            }
        }
        Err(err) => error!(?err, "Failed to read services"),
    }

    std::process::Command::new("/bin/cog")
        .env("PATH", "/bin")
        .spawn()?
        .wait()?;

    loop {}
}

fn reaper() {
    loop {
        let result = rustix::process::waitpid(None, WaitOptions::empty());
        if let Err(err) = result {
            if err == Errno::CHILD {
                // No children.. maybe we're gonna get children in the future?
                // TODO: this is probably unnecessary if we do pid 1 properly?
                std::thread::sleep(Duration::from_secs(1));
                continue;
            }
            error!(?err, "Failed to waitpid");
            return;
        }
    }
}

#[derive(Deserialize)]
struct Service {
    name: String,
    exec: Vec<String>,
}

fn run_service(service: &Service) -> Result<()> {
    info!(name = %service.name, "Starting service");

    std::fs::create_dir_all("/var/logs").wrap_err("creating log base directory")?;
    let logs = File::options()
        .create(true)
        .append(true)
        .open(Path::new("/var/logs").join(&service.name))
        .wrap_err("opening log file")?;

    let mut backoff_ms = 1;
    loop {
        let mut cmd = std::process::Command::new(&service.exec[0]);
        cmd.args(&service.exec[1..]);

        cmd.stdin(Stdio::null());
        cmd.stdout(Stdio::from(logs.try_clone().wrap_err("duping log file")?));
        cmd.stderr(Stdio::from(logs.try_clone().wrap_err("duping log file")?));

        let mut child = cmd
            .spawn()
            .wrap_err_with(|| format!("spawning command {}", service.exec[0]))?;

        let result = child.wait().wrap_err("waiting for child failed")?;
        if result.success() {
            info!(name = %service.name, "Service finished, restarting in {backoff_ms}ms");
        } else {
            info!(name = %service.name, code = %result.code().unwrap_or(-1), "Service errored, restarting in {backoff_ms}ms");
        }
        std::thread::sleep(Duration::from_millis(backoff_ms));
        backoff_ms *= 10;
    }
}

fn read_services(path: &Path) -> Result<(Vec<Service>, Vec<eyre::Report>)> {
    let service_files =
        std::fs::read_dir(path).wrap_err_with(|| format!("reading {}", path.display()))?;

    let mut services: Vec<Service> = vec![];
    let mut errors = vec![];

    for service_file in service_files {
        match read_service_file(service_file) {
            Ok(service) => {
                if services
                    .iter()
                    .any(|old_service| old_service.name == service.name)
                {
                    errors.push(eyre!("service name found twice: {}", service.name))
                } else {
                    services.push(service)
                }
            }
            Err(err) => errors.push(err),
        }
    }

    Ok((services, errors))
}

fn read_service_file(service_file: std::io::Result<DirEntry>) -> Result<Service> {
    let service_file = service_file.wrap_err("failed to read services")?.path();
    let service_config = std::fs::read(&service_file)
        .wrap_err_with(|| format!("reading {}", service_file.display()))?;
    let service_config = std::str::from_utf8(&service_config).wrap_err_with(|| {
        format!(
            "service config for {} is invalid UTF-8",
            service_file.display()
        )
    })?;
    let service = toml::from_str::<Service>(service_config)
        .wrap_err_with(|| format!("service config for {} is invalid", service_file.display()))?;

    if service.exec.len() < 1 {
        bail!(
            "service config for {} is invalid, must contain at least one element in exec",
            service_file.display()
        );
    }

    Ok(service)
}
