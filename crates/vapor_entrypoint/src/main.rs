//! Steam-facing terminal entrypoint for Vapor.
//!
//! This binary deliberately does not understand Vapor launch targets. Steam
//! starts this executable, it opens the platform terminal, starts the existing
//! `bin/vapor-launch.*` script with the internal `--hold` wrapper flag,
//! forwards Steam's launch arguments after that flag, waits for that terminal
//! to close, and exits with the terminal status.

#![cfg_attr(windows, windows_subsystem = "windows")]

use std::{
    env,
    ffi::OsString,
    fs::{self, File, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
    process::{Command, ExitStatus, Stdio},
    time::SystemTime,
};

#[cfg(windows)]
use std::ffi::OsStr;
#[cfg(windows)]
use std::os::windows::process::CommandExt;

fn main() {
    let status = match run() {
        Ok(status) => status.code().unwrap_or(1),
        Err(error) => {
            let _ = writeln!(std::io::stderr(), "vapor-entrypoint: {error}");
            1
        }
    };
    std::process::exit(status);
}

fn run() -> Result<ExitStatus, String> {
    let executable = env::current_exe()
        .map_err(|error| format!("failed to resolve vapor-entrypoint executable: {error}"))?;
    let app_root = discover_app_root(&executable).ok_or_else(|| {
        format!(
            "could not discover app root from executable '{}'",
            executable.display()
        )
    })?;
    let app_root = shell_safe_app_root(app_root);
    let mut log = EntryLog::open(&app_root);
    let arguments: Vec<OsString> = env::args_os().skip(1).collect();
    log.write(format!(
        "entrypoint executable={} app_root={} args={:?}",
        executable.display(),
        app_root.display(),
        redacted_arguments(&arguments)
    ));

    let script = platform_script(&app_root);
    if !script.is_file() {
        let message = format!("launch script is missing: {}", script.display());
        log.write(&message);
        return Err(message);
    }

    launch_terminal(&app_root, &script, &arguments, &mut log)
}

include!("entrypoint/app_root.rs");
include!("entrypoint/terminal.rs");
include!("entrypoint/redaction.rs");
include!("entrypoint/log.rs");
