use std::io::Error as IoError;
use std::process::Termination;
use std::process::{Command, ExitCode, ExitStatus, Stdio};

enum InheritedTermination {
    Io(IoError),
    Exit(ExitStatus),
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
