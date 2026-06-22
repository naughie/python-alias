use std::io::Error as IoError;
use std::process::Termination;
use std::process::{Command, ExitCode, ExitStatus, Stdio};

enum InheritedTermination {
    Io(IoError),
    Exit(ExitStatus),
}

fn print_signal(signal: i32) {
    fn signal_to_err(signal: i32) -> Option<(&'static str, char)> {
        if (1..=31).contains(&signal) {
            let name = match signal {
                1 => "SIGHUP",
                2 => "SIGINT",
                3 => "SIGQUIT",
                4 => "SIGILL",
                5 => "SIGTRAP",
                6 => "SIGABRT",
                8 => "SIGFPE",
                9 => "SIGKILL",
                11 => "SIGSEGV",
                13 => "SIGPIPE",
                14 => "SIGALRM",
                15 => "SIGTERM",
                _ => "unknown signal",
            };

            Some((name, (signal as u8 + (b'A' - 1)) as char))
        } else {
            None
        }
    }

    if let Some((name, c)) = signal_to_err(signal) {
        eprintln!("Signal caught: ^{c} ({name})");
    } else {
        eprintln!("Signal caught: {signal} (unknown signal)");
    }
}

impl Termination for InheritedTermination {
    fn report(self) -> ExitCode {
        use std::os::unix::process::ExitStatusExt as _;

        match self {
            Self::Io(e) => <Result<(), _> as Termination>::report(Err(e)),
            Self::Exit(status) => {
                if let Some(code) = status.code() {
                    ExitCode::from(code as u8)
                } else if let Some(signal) = status.signal() {
                    print_signal(signal);
                    ExitCode::from((128 + signal) as u8)
                } else {
                    ExitCode::FAILURE
                }
            }
        }
    }
}

fn main() -> InheritedTermination {
    Command::new("uv")
        .arg("run")
        .args(std::env::args_os().skip(1))
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .and_then(|mut child| child.wait())
        .map_or_else(InheritedTermination::Io, InheritedTermination::Exit)
}
