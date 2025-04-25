// Data structures for issues and state

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Issue {
    pub id: String,
    pub title: String,
    pub content: String,
    pub labels: Vec<String>,
    pub state: State,
    pub comments: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_issue_state_to_string() {
        assert_eq!(State::Open.to_string(), "open");
        assert_eq!(State::Closed.to_string(), "closed");
    }

    #[test]
    fn test_issue_struct() {
        let issue = Issue {
            id: "001".to_string(),
            title: "Test".to_string(),
            content: "Body".to_string(),
            labels: vec!["bug".to_string()],
            state: State::Open,
            comments: vec!["First comment".to_string()],
        };
        assert_eq!(issue.id, "001");
        assert_eq!(issue.state, State::Open);
        assert_eq!(issue.labels, vec!["bug"]);
        assert_eq!(issue.comments.len(), 1);
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum State {
    Open,
    Closed,
}

impl ToString for State {
    fn to_string(&self) -> String {
        match self {
            State::Open => "open".into(),
            State::Closed => "closed".into(),
        }
    }
}
