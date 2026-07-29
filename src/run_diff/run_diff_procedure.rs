use crate::configs::Configs;
use crate::errors::BackupError;

use super::subprocesses::{Usage, get_disk_usages};

fn unpack_stdout(stdout: &str) -> Result<(usize, String), std::num::ParseIntError> {
    let parts: Vec<&str> = stdout.split_whitespace().collect();

    let bytes = match parts.first() {
        Some(val) => val.parse::<usize>()?,
        None => 0,
    };

    let path = match parts.get(1) {
        Some(val) => val.to_string(),
        None => String::from("-"),
    };

    Ok((bytes, path))
}

fn bytes_to_human_readable(usage_bytes: usize) -> String {
    let units = ["B", "K", "M", "G", "T", "P", "E", "Z", "Y"];
    let mut size = usage_bytes as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < units.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    if usage_bytes < 1024 {
        format!("{}{}", usage_bytes, units[unit_index])
    } else {
        format!("{:.1}{}", size, units[unit_index])
    }
}

fn display_usages(usages: &Vec<Usage>) -> Result<(), BackupError> {
    println!(
        "{:<20} {:<25} {:<16} Usage",
        "Host", "Path", "Usage (bytes)"
    );
    println!(
        "{:<20} {:<25} {:<16} -------",
        "-------------------", "------------------------", "---------------"
    );

    for usage in usages {
        if let Usage::Success { host, stdout } = usage {
            let (usage_bytes, path) = unpack_stdout(stdout)?;
            let usage_bytes_human_readable = bytes_to_human_readable(usage_bytes);
            println!("{host:<20} {path:<25} {usage_bytes:<16} {usage_bytes_human_readable}");
        }
    }

    Ok(())
}

fn display_failed_usages(usages: &Vec<Usage>) {
    println!("{:<20} {:<25}", "Host", "Error");
    println!(
        "{:<20} {:<25}",
        "-------------------", "------------------------"
    );

    for usage in usages {
        if let Usage::Failure { host, stderr } = usage {
            println!("{host:<20} {stderr:<25}");
        }
    }
}

pub fn run_diff_procedure(configs: &Configs) -> Result<(), BackupError> {
    let usages = get_disk_usages(configs)?;

    display_usages(&usages)?;
    println!();
    display_failed_usages(&usages);

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_unpack_stdout_valid_cases() {
        assert_eq!(
            super::unpack_stdout("16411   /tmp/bar/"),
            Ok((16411, "/tmp/bar/".into()))
        );
        assert_eq!(
            super::unpack_stdout("   16411   /tmp/bar/"),
            Ok((16411, "/tmp/bar/".into()))
        );
        assert_eq!(
            super::unpack_stdout("16411   /tmp/bar/   "),
            Ok((16411, "/tmp/bar/".into()))
        );
        assert_eq!(super::unpack_stdout("16411 "), Ok((16411, "-".into())));
        assert_eq!(super::unpack_stdout("16411"), Ok((16411, "-".into())));
        assert_eq!(super::unpack_stdout(" "), Ok((0, "-".into())));
        assert_eq!(super::unpack_stdout(""), Ok((0, "-".into())));
    }

    #[test]
    fn test_unpack_stdout_not_parsable() {
        assert!(matches!(super::unpack_stdout("?????   /tmp/bar/"), Err(_)));
    }

    #[test]
    fn test_bytes_to_human_readable() {
        assert_eq!(super::bytes_to_human_readable(0), "0B");
        assert_eq!(super::bytes_to_human_readable(512), "512B");
        assert_eq!(super::bytes_to_human_readable(16411), "16.0K");
        assert_eq!(super::bytes_to_human_readable(16900), "16.5K");
        assert_eq!(super::bytes_to_human_readable(2174555), "2.1M");
        assert_eq!(super::bytes_to_human_readable(107916553087), "100.5G");
    }
}
