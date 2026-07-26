use std::io::{self, Write};
use std::str::FromStr;

fn read_option_from_stdin() -> i32 {
    print!("> ");
    io::stdout()
        .flush()
        .expect("Unrecoverable error: Failed to flush stdout");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Unrecoverable error: Failed to read from stdin");

    match i32::from_str(input.trim()) {
        Ok(val) => val,
        Err(_) => 0,
    }
}

pub fn select_backup_type() -> (bool, bool, bool) {
    println!("Select backup type:");
    println!("[1] -> Synchronize directories to HOT storage");
    println!("[2] -> Synchronize directories to HOT storage [DRY RUN]");
    println!("[3] -> Synchronize directories to COLD storage");
    println!("[4] -> Synchronize directories to COLD storage [DRY RUN]");
    println!("[*] -> Exit program");

    let option = read_option_from_stdin();

    let sync_to_hot = matches!(option, 1 | 2);
    let is_dry_run = matches!(option, 2 | 4);
    let exit_program = option < 1 || option > 4;

    (sync_to_hot, is_dry_run, exit_program)
}
