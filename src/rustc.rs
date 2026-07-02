use clap::{Error, ErrorKind};
use std::{io::BufRead, process::Command};

/// Queries `rustc` to identify the host platform's target triple.
pub fn host_triple() -> String {
    Command::new("rustc")
        .arg("-vV")
        .output()
        .map_err(|err| err.to_string())
        .and_then(|out| {
            // Look for a line starting with `host: `, just like `cargo` does, to identify the
            // host platform -- see:
            // https://github.com/rust-lang/cargo/blob/631b8774e512a69402a55367b4eb9c195220e404/src/cargo/util/rustc.rs#L68
            out.stdout
                .lines()
                .map_while(Result::ok)
                .find_map(|line| line.strip_prefix("host: ").map(ToString::to_string))
                .ok_or_else(|| "`rustc -vV` didn't have a line for `host`.".to_string())
        })
        .unwrap_or_else(|err| Error::with_description(&err, ErrorKind::Io).exit())
}
