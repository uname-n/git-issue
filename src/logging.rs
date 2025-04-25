// Audit log handling utilities

use anyhow::{Context, Result};
use std::fs::{OpenOptions, File};
use std::io::{Write, BufRead, BufReader};
use std::path::PathBuf;
use crate::storage::STORAGE_DIR;

pub const AUDIT_LOG: &str = "audit.log";

/// Append a single-line entry (no timestamp) to `.issues/audit.log`
pub fn append_log(entry: &str) -> Result<()> {
    let mut path = PathBuf::from(STORAGE_DIR);
    path.push(AUDIT_LOG);
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)
        .context("opening audit log")?;
    writeln!(file, "{}", entry)?;
    Ok(())
}

/// Read and print the audit log in descending order, applying `limit` if present
pub fn show_log(limit: Option<usize>) -> Result<()> {
    let mut path = PathBuf::from(STORAGE_DIR);
    path.push(AUDIT_LOG);
    let file = File::open(&path).context("opening audit log for reading")?;
    let reader = BufReader::new(file);

    let mut lines: Vec<_> = reader.lines().collect::<Result<_, _>>()?;
    lines.reverse();

    if let Some(limit) = limit {
        for line in lines.into_iter().take(limit) {
            println!("{}", line);
        }
    } else {
        for line in lines {
            println!("{}", line);
        }
    }

    Ok(())
}
