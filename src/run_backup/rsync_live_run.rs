use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::prelude::*;
use tracing_subscriber::{Registry, fmt};

use crate::errors::BackupError;
use crate::program_files;

use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::process::{ChildStderr, ChildStdout, Command, Stdio};
use std::thread;

fn worker_log_stdout(stdout: ChildStdout) {
    for line in BufReader::new(stdout).lines() {
        match line {
            Ok(text) => tracing::info!("(stdout) {text}"),
            Err(e) => tracing::error!("Error reading line from stdout: {e}"),
        }
    }
}

fn worker_log_stderr(stderr: ChildStderr) {
    for line in BufReader::new(stderr).lines() {
        match line {
            Ok(text) => tracing::error!("(stderr) {text}"),
            Err(e) => tracing::error!("Error reading line from stderr: {e}"),
        }
    }
}

pub fn run_rsync(src: &str, user: &String, host: &String, dst: &String) -> Result<(), BackupError> {
    tracing::info!("Starting data synchronization");

    let dst = format!("{user}@{host}:{dst}");

    let mut child = Command::new("rsync")
        .args(["-av", "--delete", src, &dst])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let stdout = child
        .stdout
        .take()
        .ok_or_else(|| io::Error::other("Could not capture stdout"))?;

    let stderr = child
        .stderr
        .take()
        .ok_or_else(|| io::Error::other("Could not capture stderr"))?;

    let handle_stdout = thread::spawn(move || worker_log_stdout(stdout));
    let handle_stderr = thread::spawn(move || worker_log_stderr(stderr));

    if handle_stdout.join().is_err() {
        return Err(BackupError::IO {
            source: io::Error::other("The stdout thread failed"),
        });
    }

    if handle_stderr.join().is_err() {
        return Err(BackupError::IO {
            source: io::Error::other("The stderr thread failed"),
        });
    }

    let status = child.wait()?;

    if status.success() {
        tracing::info!("Synchronization succeeded\n");
    } else {
        tracing::error!("Synchronization failed\n");
    }

    Ok(())
}

pub fn run_rsync_subprocess(
    src: &str,
    user: &String,
    host: &String,
    dst: &String,
    log_file: &Path,
) -> Result<(), BackupError> {
    let stdout_layer = fmt::layer()
        .with_writer(io::stdout)
        .with_target(false)
        .with_filter(LevelFilter::DEBUG);

    let app_dir = program_files::get_app_dir()?;
    let log_dir = program_files::get_log_dir(&app_dir);
    let file_appender = tracing_appender::rolling::never(log_dir, log_file);
    let (non_blocking_file, _guard) = tracing_appender::non_blocking(file_appender);
    // every scope where logging takes place (this + children) must have access to _guard

    let file_layer = fmt::layer()
        .with_writer(non_blocking_file)
        .with_target(false)
        .with_ansi(false)
        .with_filter(LevelFilter::DEBUG);

    Registry::default()
        .with(stdout_layer)
        .with(file_layer)
        .init();

    run_rsync(src, user, host, dst)
}
