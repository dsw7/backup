use std::io::{self, Write};
use std::str::FromStr;

fn print_prompt() {
    print!("> ");
    io::stdout()
        .flush()
        .expect("Unrecoverable error: Failed to flush stdout");
}

fn read_option_from_stdin() -> Option<i32> {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Unrecoverable error: Failed to read from stdin");

    let option = match i32::from_str(input.trim()) {
        Ok(val) => val,
        Err(_) => {
            println!("Cannot convert input to integer value");
            return None;
        }
    };

    if option < 1 || option > 5 {
        println!("Enter an option between 1 and 5");
        return None;
    }

    Some(option)
}

fn run_option_loop() -> i32 {
    loop {
        print_prompt();

        match read_option_from_stdin() {
            Some(option) => return option,
            None => {
                println!("Try again");
                continue;
            }
        }
    }
}

pub fn select_backup_type() -> (bool, bool, bool) {
    println!("Select backup type:");
    println!("[1] -> Synchronize directories to HOT storage");
    println!("[2] -> Synchronize directories to HOT storage [DRY RUN]");
    println!("[3] -> Synchronize directories to COLD storage");
    println!("[4] -> Synchronize directories to COLD storage [DRY RUN]");
    println!("[5] -> Exit program");

    let option = run_option_loop();
    let sync_to_hot = matches!(option, 1 | 2);
    let is_dry_run = matches!(option, 2 | 4);
    let exit_program = option == 5;

    (sync_to_hot, is_dry_run, exit_program)
}
