use std::process::Command;

pub fn run_backup(src: &String, dst: &String) {
    let status = Command::new("rsync")
        .arg("-av")
        .arg("--dry-run")
        .arg("--delete")
        .arg(src)
        .arg(dst)
        .status()
        .expect("Command failed to start. There is no way to proceed");

    println!("Process exited with status: {}", status);
}
