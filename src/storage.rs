// Storage and file I/O utilities for issues

use std::fs;
use std::io::Write;
use std::path::PathBuf;
use anyhow::{Context, Result};
use crate::models::Issue;

pub const STORAGE_DIR: &str = ".issues";

/// Compute file path for an issue ID, nesting sub-issues in `parent/child.yaml`
pub fn path_for(id: &str) -> PathBuf {
    let mut path = PathBuf::from(STORAGE_DIR);
    if let Some((parent, _)) = id.split_once('-') {
        path.push(parent);
        path.push(format!("{}.yaml", id));
    } else {
        path.push(format!("{}.yaml", id));
    }
    path
}

/// Save issue back to storage, creating parent directory if needed
pub fn save(issue: &Issue) -> Result<()> {
    let path = path_for(&issue.id);
    if let Some(parent_dir) = path.parent() {
        fs::create_dir_all(parent_dir)?;
    }
    let mut file = fs::File::create(&path)
        .with_context(|| format!("Failed to save issue {}", issue.id))?;
    let yaml = serde_yaml::to_string(issue)?;
    file.write_all(yaml.as_bytes())?;
    Ok(())
}

/// Load issue from storage
pub fn load(id: &str) -> Result<Issue> {
    let path = path_for(id);
    let data =
        fs::read_to_string(&path).with_context(|| format!("Failed to read issue {}", id))?;
    let issue: Issue = serde_yaml::from_str(&data)?;
    Ok(issue)
}

/// Determine next root issue ID
pub fn next_root_id() -> Result<String> {
    let mut max_id = 0;
    for entry in fs::read_dir(STORAGE_DIR)? {
        let name = entry?.file_name().into_string().unwrap();
        if let Some(base) = name.strip_suffix(".yaml") {
            if base.len() == 3 && base.chars().all(|c| c.is_digit(10)) {
                let v: usize = base.parse()?;
                max_id = max_id.max(v);
            }
        }
    }
    Ok(format!("{:03}", max_id + 1))
}

/// Determine next sub-issue ID under given parent, scanning `.issues/{parent}`
pub fn next_child_id(parent: &str) -> Result<String> {
    let mut max_child = 0;
    let dir = PathBuf::from(STORAGE_DIR).join(parent);
    if dir.exists() {
        for entry in fs::read_dir(&dir)? {
            let name = entry?.file_name().into_string().unwrap();
            if let Some(base) = name.strip_suffix(".yaml") {
                if let Some(suffix) = base.strip_prefix(&format!("{}-", parent)) {
                    if let Ok(v) = suffix.parse::<usize>() {
                        max_child = max_child.max(v);
                    }
                }
            }
        }
    }
    Ok(format!("{}-{:03}", parent, max_child + 1))
}
