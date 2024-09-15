use std::io::Write;

fn main() -> std::io::Result<()> {
    let mut last_success = true;
    loop {
        let mut line = String::new();

        print!("{}$ ", if last_success { "".into() } else { "[error]" });
        std::io::stdout().flush()?;
        std::io::stdin().read_line(&mut line)?;

        if !line.trim().is_empty() {
            match exec_line(&line) {
                Ok(success) => last_success = success,
                Err(err) => {
                    eprintln!("{err}");
                }
            }
        }
    }
}

fn exec_line(line: &str) -> Result<bool, String> {
    let commands = shlex::split(line).ok_or_else(|| "invalid command".to_owned())?;
    if commands.len() < 1 {
        return Err("invalid command".to_owned());
    }

    let arg0 = &commands[0];

    if arg0 == "exit" {
        std::process::exit(0);
    }

    let mut cmd = std::process::Command::new(arg0);
    cmd.args(&commands[1..]);

    let mut cmd = cmd
        .spawn()
        .map_err(|err| format!("failed to spawn {arg0}: {err}"))?;

    let result = cmd
        .wait()
        .map_err(|err| format!("failed to wait for {arg0}: {err}"))?;

    Ok(result.success())
}
