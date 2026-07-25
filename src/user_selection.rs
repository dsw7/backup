use std::io::{self, Write};
use std::str::FromStr;

fn read_option_from_stdin() -> Option<i32> {
    print!("> ");
    io::stdout()
        .flush()
        .expect("Unrecoverable error: Failed to flush stdout");

    let mut input = String::new();

    if let Err(e) = io::stdin().read_line(&mut input) {
        eprintln!("{e}");
        return None;
    }

    let option = match i32::from_str(input.trim()) {
        Ok(val) => val,
        Err(e) => {
            eprintln!("{e}");
            return None;
        }
    };

    if !matches!(option, 1 | 2 | 3 | 4) {
        eprintln!("Enter an option between 1 and 4");
        return None;
    }

    Some(option)
}

fn run_loop() -> i32 {
    loop {
        match read_option_from_stdin() {
            Some(option) => return option,
            None => continue,
        }
    }
}

pub fn select_backup_type() -> i32 {
    println!("Select backup type:");
    println!("[1] -> Synchronize directories to HOT storage");
    println!("[2] -> Synchronize directories to HOT storage [DRY RUN]");
    println!("[3] -> Synchronize directories to COLD storage");
    println!("[4] -> Synchronize directories to COLD storage [DRY RUN]");

    run_loop()
}
