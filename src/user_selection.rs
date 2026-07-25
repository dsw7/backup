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

pub struct UserSelection {
    sync_to_hot: bool,
    is_dry_run: bool,
    exit_program: bool,
}

fn get_user_selection(option: i32) -> UserSelection {
    let mut sync_to_hot = false;
    let mut is_dry_run = false;
    let mut exit_program = false;

    match option {
        1 => sync_to_hot = true,
        2 => {
            sync_to_hot = true;
            is_dry_run = true;
        }
        4 => is_dry_run = true,
        5 => exit_program = true,
        _ => unreachable!("Value is checked upstream"),
    }

    UserSelection {
        sync_to_hot: sync_to_hot,
        is_dry_run: is_dry_run,
        exit_program: exit_program,
    }
}

pub fn select_backup_type() -> UserSelection {
    println!("Select backup type:");
    println!("[1] -> Synchronize directories to HOT storage");
    println!("[2] -> Synchronize directories to HOT storage [DRY RUN]");
    println!("[3] -> Synchronize directories to COLD storage");
    println!("[4] -> Synchronize directories to COLD storage [DRY RUN]");
    println!("[5] -> Exit program");

    let option = run_option_loop();
    get_user_selection(option)
}
