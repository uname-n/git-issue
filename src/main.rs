use anyhow::Result;
use clap::{Parser, Subcommand};
use git_issue::commands;
use git_issue::logging::{append_log, show_log};
use std::fs;

const STORAGE_DIR: &str = ".issues";

#[derive(Parser)]
#[command(name = "git-issue", version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new issue or sub-issue
    Create(git_issue::CreateArgs),
    /// List issues
    Ls(git_issue::LsArgs),
    /// View an issue and its details
    View { id: String },
    /// Add a comment
    Comment(git_issue::CommentArgs),
    /// Close an issue
    Close(git_issue::CloseArgs),
    /// Reopen an issue
    Reopen(git_issue::CloseArgs),
    /// Show write-only audit trail
    Log(git_issue::LogArgs),
    /// Batch create issues and sub-issues from JSON
    Plan(git_issue::PlanArgs),
}




fn main() -> Result<()> {
    let cli = Cli::parse();
    fs::create_dir_all(STORAGE_DIR)?;

    match cli.command {
        Commands::Create(args) => {
            let issue = commands::create(args)?;
            append_log(&format!("CREATE id={} title={}", issue.id, issue.title))?;
        }
        Commands::Ls(args) => commands::list(args)?,
        Commands::View { id } => commands::view(&id)?,
        Commands::Comment(args) => {
            commands::comment(&args.id, &args.message)?;
            append_log(&format!("COMMENT id={} msg={}", args.id, args.message))?;
        }
        Commands::Close(args) => {
            commands::close(&args.id, &args.message)?;
            append_log(&format!("CLOSE id={} msg={}", args.id, args.message))?;
        }
        Commands::Reopen(args) => {
            commands::reopen(&args.id, &args.message)?;
            append_log(&format!("REOPEN id={} msg={}", args.id, args.message))?;
        }
        Commands::Log(args) => show_log(args.limit)?,
        Commands::Plan(args) => {
            let (parent_id, parent_title) = commands::plan(args.clone())?;
            append_log(&format!("PLAN parent_id={} parent_title={}", parent_id, parent_title))?;
        }
    }

    Ok(())
}
