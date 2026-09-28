use std::{
    fs,
    net::TcpStream,
    os::unix::process::CommandExt,
    path::PathBuf,
    process::{Command, ExitCode, Stdio},
    thread::sleep,
    time::Duration,
};

use loco_app::app::App;
use loco_rs::cli;
use migration::Migrator;

/// Two additions to Loco's CLI: `start -d` (or `--detach`) runs the same `start` in the
/// background, and `stop` ends it. The pid and the log sit beside the binary, in
/// `target/debug/loco-app.pid` and `target/debug/loco-app.log`.
fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let detach = |a: &String| a == "-d" || a == "--detach";
    match args.first().map(String::as_str) {
        Some("start") if args.iter().any(detach) => {
            detached(args.into_iter().filter(|a| !detach(a)).collect())
        }
        Some("stop") => stop(),
        _ => run(),
    }
}

#[tokio::main]
async fn run() -> ExitCode {
    match cli::main::<App, Migrator>().await {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("Error: {e:?}");
            ExitCode::FAILURE
        }
    }
}

fn file(ext: &str) -> PathBuf {
    let exe = std::env::current_exe().expect("the binary's path");
    exe.with_file_name(format!("loco-app.{ext}"))
}

/// The pid in the pid file, if that process is still alive.
fn running() -> Option<String> {
    let pid = fs::read_to_string(file("pid")).ok()?.trim().to_string();
    let alive = Command::new("kill")
        .args(["-0", &pid])
        .stderr(Stdio::null())
        .status();
    alive.is_ok_and(|s| s.success()).then_some(pid)
}

fn detached(args: Vec<String>) -> ExitCode {
    let port = (args.iter().position(|a| a == "-p" || a == "--port"))
        .and_then(|i| args.get(i + 1).cloned())
        .unwrap_or_else(|| "5150".into());
    let url = format!("http://localhost:{port}/");
    let answers = || TcpStream::connect(("127.0.0.1", port.parse().unwrap_or(5150))).is_ok();
    if let Some(pid) = running() {
        println!("already running (pid {pid}) at {url}");
        return ExitCode::SUCCESS;
    }
    if answers() {
        eprintln!("something else already answers at {url}");
        return ExitCode::FAILURE;
    }
    let log = file("log");
    let out = fs::File::create(&log).expect("the log file");
    let mut child = Command::new(std::env::current_exe().expect("the binary's path"))
        .args(&args)
        .stdin(Stdio::null())
        .stdout(out.try_clone().expect("the log file"))
        .stderr(out)
        .process_group(0)
        .spawn()
        .expect("starting the server");
    fs::write(file("pid"), child.id().to_string()).expect("the pid file");
    for _ in 0..120 {
        if let Ok(Some(_)) = child.try_wait() {
            let text = fs::read_to_string(&log).unwrap_or_default();
            eprintln!("exited; see {}\n{text}", log.display());
            return ExitCode::FAILURE;
        }
        if answers() {
            println!(
                "running (pid {}) at {url}; log in {}",
                child.id(),
                log.display()
            );
            return ExitCode::SUCCESS;
        }
        sleep(Duration::from_millis(250));
    }
    println!(
        "started (pid {}) but not answering yet; see {}",
        child.id(),
        log.display()
    );
    ExitCode::SUCCESS
}

fn stop() -> ExitCode {
    match running() {
        Some(pid) => {
            let _ = Command::new("kill").arg(&pid).status();
            println!("stopped (pid {pid})");
        }
        None => println!("not running"),
    }
    let _ = fs::remove_file(file("pid"));
    ExitCode::SUCCESS
}
