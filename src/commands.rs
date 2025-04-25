// Business logic for CLI commands

use crate::models::{Issue, State};
use crate::storage::{save, load, next_root_id, next_child_id, STORAGE_DIR};
use anyhow::{Result};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

pub fn create(args: crate::CreateArgs) -> Result<Issue> {
    let id = if let Some(parent) = args.parent.clone() {
        load(&parent)?; // ensure parent exists
        next_child_id(&parent)?
    } else {
        next_root_id()?
    };

    let labels = args.label.unwrap_or_default();

    let issue = Issue {
        id: id.clone(),
        title: args.title,
        content: args.content,
        labels,
        state: State::Open,
        comments: Vec::new(),
    };

    save(&issue)?;

    // Print summary
    if !issue.labels.is_empty() {
        println!("{} | {} - {}", issue.id, issue.title, issue.labels.join(","));
    } else {
        println!("{} | {}", issue.id, issue.title);
    }

    Ok(issue.clone())
}

pub fn list(args: crate::LsArgs) -> Result<()> {
    let mut roots: Vec<Issue> = Vec::new();
    let mut children_map: HashMap<String, Vec<Issue>> = HashMap::new();

    for entry in fs::read_dir(STORAGE_DIR)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            let name = entry.file_name().into_string().unwrap();
            if let Some(id) = name.strip_suffix(".yaml") {
                let issue = load(id)?;
                if args.state != "all" && issue.state.to_string() != args.state {
                    continue;
                }
                if let Some(ref filter) = args.label {
                    let filter = filter.trim().to_lowercase();
                    if !issue.labels.iter().any(|l| l.trim().to_lowercase() == filter) {
                        continue;
                    }
                }
                roots.push(issue);
            }
        } else if path.is_dir() {
            let parent = entry.file_name().into_string().unwrap();
            for child_entry in fs::read_dir(&path)? {
                let child_entry = child_entry?;
                let fname = child_entry.file_name().into_string().unwrap();
                if let Some(child_id) = fname.strip_suffix(".yaml") {
                    let issue = load(child_id)?;
                    if args.state != "all" && issue.state.to_string() != args.state {
                        continue;
                    }
                    if let Some(ref filter) = args.label {
                        let filter = filter.trim().to_lowercase();
                        if !issue.labels.iter().any(|l| l.trim().to_lowercase() == filter) {
                            continue;
                        }
                    }
                    children_map.entry(parent.clone()).or_default().push(issue);
                }
            }
        }
    }

    roots.sort_by_key(|iss| iss.id.clone());
    if args.order == "desc" {
        roots.reverse();
    }

    for root in roots {
        let base = if !root.labels.is_empty() {
            format!("{} | {} - {}", root.id, root.title, root.labels.join(","))
        } else {
            format!("{} | {}", root.id, root.title)
        };
        if root.state == State::Closed {
            println!("{} [closed]", base);
        } else {
            println!("{}", base);
        }
        if let Some(mut children) = children_map.remove(&root.id) {
            children.sort_by_key(|c| c.id.clone());
            for child in children {
                let indent = "    ";
                let base = if !child.labels.is_empty() {
                    format!(
                        "{}{} | {} - {}",
                        indent,
                        child.id,
                        child.title,
                        child.labels.join(",")
                    )
                } else {
                    format!("{}{} | {}", indent, child.id, child.title)
                };
                if child.state == State::Closed {
                    println!("{} [closed]", base);
                } else {
                    println!("{}", base);
                }
            }
        }
    }

    Ok(())
}

pub fn view(id: &str) -> Result<()> {
    let issue = load(id)?;
    if !issue.labels.is_empty() {
        println!("{} | {} - {}", issue.id, issue.title, issue.labels.join(","));
    } else {
        println!("{} | {}", issue.id, issue.title);
    }
    println!("\n{}\n", issue.content);

    // Show any children under .issues/{id}/
    let dir = PathBuf::from(STORAGE_DIR).join(id);
    if dir.exists() {
        let mut children_ids: Vec<String> = fs::read_dir(dir)?
            .filter_map(|e| e.ok())
            .filter_map(|e| {
                let fname = e.file_name().into_string().ok()?;
                fname.strip_suffix(".yaml").map(String::from)
            })
            .collect();
        children_ids.sort();
        if !children_ids.is_empty() {
            println!("@ref{{{}}}", children_ids.join(", "));
        }
    }

    for comment in issue.comments {
        println!("{}", comment);
    }
    Ok(())
}

pub fn append_comment(id: &str, entry: &str) -> Result<()> {
    let mut issue = load(id)?;
    issue.comments.push(entry.to_string());
    save(&issue)
}

pub fn comment(id: &str, message: &str) -> Result<()> {
    let entry = format!("+++ {}", message);
    append_comment(id, &entry)?;
    println!("{} | {}", id, entry);
    Ok(())
}

pub fn close(id: &str, message: &str) -> Result<()> {
    let child_dir = PathBuf::from(STORAGE_DIR).join(id);
    if child_dir.exists() {
        for entry in fs::read_dir(child_dir)? {
            let entry = entry?;
            let fname = entry.file_name().into_string().unwrap();
            if let Some(child_id) = fname.strip_suffix(".yaml") {
                let child = load(child_id)?;
                if child.state == State::Open {
                    eprintln!("error: child issues are still pending");
                    std::process::exit(1);
                }
            }
        }
    }
    let entry = format!(">>> {}", message);
    append_comment(id, &entry)?;
    let mut issue = load(id)?;
    issue.state = State::Closed;
    save(&issue)?;
    println!("{} | >>> {}", id, message);
    Ok(())
}

pub fn reopen(id: &str, message: &str) -> Result<()> {
    let mut issue = load(id)?;
    if issue.state != State::Closed {
        eprintln!("error: issue is not closed");
        std::process::exit(1);
    }

    if let Some((parent, _)) = id.split_once('-') {
        let p_issue = load(parent)?;
        if p_issue.state == State::Closed {
            eprintln!("error: parent issue closed");
            std::process::exit(1);
        }
    }

    issue.state = State::Open;
    let entry = format!("<<< {}", message);
    issue.comments.push(entry);
    save(&issue)?;
    println!("{} | <<< {}", id, message);
    Ok(())
}
