pub mod models;
pub mod storage;
pub mod commands;
pub mod logging;

use clap::Args;

#[derive(Args, Debug, Clone)]
pub struct CreateArgs {
    /// Parent issue ID (for sub-issue)
    #[arg(short = 'p', long)]
    pub parent: Option<String>,
    /// Title of the issue
    #[arg(short = 't', long)]
    pub title: String,
    /// Content/body of the issue
    #[arg(short = 'c', long)]
    pub content: String,
    /// Comma-separated labels
    #[arg(long, value_delimiter = ',')]
    pub label: Option<Vec<String>>,
}

#[derive(Args, Debug, Clone)]
pub struct LsArgs {
    /// Filter by state: open, closed, or all
    #[arg(long, default_value = "open")]
    pub state: String,
    /// Filter by label
    #[arg(long)]
    pub label: Option<String>,
    /// Sort by: id
    #[arg(long, default_value = "id")]
    pub sort: String,
    /// Order: asc or desc
    #[arg(long, default_value = "asc")]
    pub order: String,
}

#[derive(Args, Debug, Clone)]
pub struct CommentArgs {
    /// Issue ID
    pub id: String,
    /// Message
    #[arg(short = 'm', long)]
    pub message: String,
}

#[derive(Args, Debug, Clone)]
pub struct CloseArgs {
    /// Issue ID
    pub id: String,
    /// Message
    #[arg(short = 'm', long)]
    pub message: String,
}

#[derive(Args, Debug, Clone)]
pub struct LogArgs {
    /// Show only the last N entries
    #[arg(short, long)]
    pub limit: Option<usize>,
}

#[derive(Args, Debug, Clone)]
pub struct PlanArgs {
    /// Path to JSON file describing the plan
    #[arg(short = 'f', long)]
    pub file: Option<std::path::PathBuf>,
    /// Inline JSON string describing the plan
    #[arg(short = 'j', long)]
    pub json: Option<String>,
}
