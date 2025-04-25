// End-to-end smoke tests for git-issue CLI

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

fn setup_temp_dir() -> tempfile::TempDir {
    tempfile::tempdir().expect("failed to create temp dir")
}

#[test]
fn test_full_usage_example_smoke() {
    let temp = setup_temp_dir();
    let issues_dir = temp.path().join(".issues");
    fs::create_dir_all(&issues_dir).unwrap();

    // 1. Create root issues
    let mut cmd = Command::cargo_bin("git-issue").unwrap();
    cmd.current_dir(&temp)
        .args(&["create", "-t", "title-1", "-c", "content", "--label", "bug,high"]);
    cmd.assert().success();

    let mut cmd = Command::cargo_bin("git-issue").unwrap();
    cmd.current_dir(&temp)
        .args(&["create", "-t", "title-2", "-c", "content", "--label", "bug"]);
    cmd.assert().success();

    let mut cmd = Command::cargo_bin("git-issue").unwrap();
    cmd.current_dir(&temp)
        .args(&["create", "-t", "title-3", "-c", "content"]);
    cmd.assert().success();

    // 2. Create sub-issues
    let mut cmd = Command::cargo_bin("git-issue").unwrap();
    cmd.current_dir(&temp)
        .args(&["create", "-p", "001", "-t", "title-1-1", "-c", "content"]);
    cmd.assert().success();

    let mut cmd = Command::cargo_bin("git-issue").unwrap();
    cmd.current_dir(&temp)
        .args(&["create", "-p", "001", "-t", "title-1-2", "-c", "content"]);
    cmd.assert().success();

    let mut cmd = Command::cargo_bin("git-issue").unwrap();
    cmd.current_dir(&temp)
        .args(&["create", "-p", "002", "-t", "title-2-1", "-c", "content"]);
    cmd.assert().success();

    // 3. List default (open)
    let mut cmd = Command::cargo_bin("git-issue").unwrap();
    cmd.current_dir(&temp).arg("ls");
    cmd.assert().success().stdout(predicate::str::contains("title-1"))
        .stdout(predicate::str::contains("title-2"))
        .stdout(predicate::str::contains("title-3"));

    // 4. List all, sorted desc
    let mut cmd = Command::cargo_bin("git-issue").unwrap();
    cmd.current_dir(&temp)
        .args(&["ls", "--state", "all", "--sort", "id", "--order", "desc"]);
    cmd.assert().success().stdout(predicate::str::contains("title-1"));

    // 5. List bug label, asc
    let mut cmd = Command::cargo_bin("git-issue").unwrap();
    cmd.current_dir(&temp)
        .args(&["ls", "--label", "bug", "--sort", "id", "--order", "asc"]);
    cmd.assert().success().stdout(predicate::str::contains("title-1"))
        .stdout(predicate::str::contains("title-2"));

    // 6. View issues
    let mut cmd = Command::cargo_bin("git-issue").unwrap();
    cmd.current_dir(&temp).args(&["view", "003"]);
    cmd.assert().success().stdout(predicate::str::contains("title-3"));

    let mut cmd = Command::cargo_bin("git-issue").unwrap();
    cmd.current_dir(&temp).args(&["view", "001"]);
    cmd.assert().success().stdout(predicate::str::contains("title-1"));

    // 7. Comments on 001
    for comment in &["comment-1", "comment-2", "comment-3"] {
        let mut cmd = Command::cargo_bin("git-issue").unwrap();
        cmd.current_dir(&temp)
            .args(&["comment", "001", "-m", comment]);
        cmd.assert().success().stdout(predicate::str::contains(*comment));
    }

    // 8. View 001 with comments
    let mut cmd = Command::cargo_bin("git-issue").unwrap();
    cmd.current_dir(&temp).args(&["view", "001"]);
    cmd.assert().success()
        .stdout(predicate::str::contains("comment-1"))
        .stdout(predicate::str::contains("comment-2"))
        .stdout(predicate::str::contains("comment-3"));

    // 9. Attempt closing parent with open children (should fail)
    let mut cmd = Command::cargo_bin("git-issue").unwrap();
    cmd.current_dir(&temp)
        .args(&["close", "001", "-m", "close-comment"]);
    cmd.assert().failure().stderr(predicate::str::contains("child issues are still pending"));

    // 10. Close sub-issues then parent
    let mut cmd = Command::cargo_bin("git-issue").unwrap();
    cmd.current_dir(&temp)
        .args(&["close", "001-001", "-m", "close-comment"]);
    cmd.assert().success();

    let mut cmd = Command::cargo_bin("git-issue").unwrap();
    cmd.current_dir(&temp)
        .args(&["close", "001-002", "-m", "close-comment"]);
    cmd.assert().success();

    let mut cmd = Command::cargo_bin("git-issue").unwrap();
    cmd.current_dir(&temp)
        .args(&["close", "001", "-m", "close-comment"]);
    cmd.assert().success();

    // 11. View 001 after close
    let mut cmd = Command::cargo_bin("git-issue").unwrap();
    cmd.current_dir(&temp).args(&["view", "001"]);
    cmd.assert().success().stdout(predicate::str::contains("title-1"));

    // 12. List open issues
    let mut cmd = Command::cargo_bin("git-issue").unwrap();
    cmd.current_dir(&temp).arg("ls");
    cmd.assert().success();

    // 13. List all issues
    let mut cmd = Command::cargo_bin("git-issue").unwrap();
    cmd.current_dir(&temp).args(&["ls", "--state", "all"]);
    cmd.assert().success().stdout(predicate::str::contains("title-1"));

    // 14. Reopen tests
    // Reopen sub-issue should fail if parent closed
    let mut cmd = Command::cargo_bin("git-issue").unwrap();
    cmd.current_dir(&temp)
        .args(&["reopen", "001-001", "-m", "reopen-comment"]);
    cmd.assert().failure().stderr(predicate::str::contains("parent issue closed"));

    // Reopen parent then child
    let mut cmd = Command::cargo_bin("git-issue").unwrap();
    cmd.current_dir(&temp)
        .args(&["reopen", "001", "-m", "reopen-comment"]);
    cmd.assert().success();

    let mut cmd = Command::cargo_bin("git-issue").unwrap();
    cmd.current_dir(&temp)
        .args(&["reopen", "001-001", "-m", "reopen-comment"]);
    cmd.assert().success();

    // 15. New comment and re-close flow
    let mut cmd = Command::cargo_bin("git-issue").unwrap();
    cmd.current_dir(&temp)
        .args(&["comment", "001", "-m", "comment-4"]);
    cmd.assert().success().stdout(predicate::str::contains("comment-4"));

    let mut cmd = Command::cargo_bin("git-issue").unwrap();
    cmd.current_dir(&temp)
        .args(&["close", "001-001", "-m", "close-comment"]);
    cmd.assert().success();

    let mut cmd = Command::cargo_bin("git-issue").unwrap();
    cmd.current_dir(&temp)
        .args(&["close", "001", "-m", "close-comment"]);
    cmd.assert().success();

    // 16. Final view
    let mut cmd = Command::cargo_bin("git-issue").unwrap();
    cmd.current_dir(&temp).args(&["view", "001"]);
    cmd.assert().success().stdout(predicate::str::contains("title-1"));
}

#[test]
fn test_comment_and_view() {
    let temp = setup_temp_dir();
    let issues_dir = temp.path().join(".issues");
    fs::create_dir_all(&issues_dir).unwrap();

    // Create an issue
    let mut cmd = Command::cargo_bin("git-issue").unwrap();
    cmd.current_dir(&temp)
        .arg("create")
        .arg("-t")
        .arg("Commented Issue")
        .arg("-c")
        .arg("Needs discussion");
    cmd.assert().success();

    // Add a comment
    let mut cmd = Command::cargo_bin("git-issue").unwrap();
    cmd.current_dir(&temp)
        .arg("comment")
        .arg("001")
        .arg("-m")
        .arg("First comment");
    cmd.assert().success().stdout(predicate::str::contains("+++ First comment"));

    // View the issue and check for comment
    let mut cmd = Command::cargo_bin("git-issue").unwrap();
    cmd.current_dir(&temp)
        .arg("view")
        .arg("001");
    cmd.assert().success().stdout(predicate::str::contains("+++ First comment"));
}

#[test]
fn test_reopen_and_error_cases() {
    let temp = setup_temp_dir();
    let issues_dir = temp.path().join(".issues");
    fs::create_dir_all(&issues_dir).unwrap();

    // Create and close an issue
    let mut cmd = Command::cargo_bin("git-issue").unwrap();
    cmd.current_dir(&temp)
        .arg("create")
        .arg("-t")
        .arg("To Reopen")
        .arg("-c")
        .arg("Should be reopened");
    cmd.assert().success();

    let mut cmd = Command::cargo_bin("git-issue").unwrap();
    cmd.current_dir(&temp)
        .arg("close")
        .arg("001")
        .arg("-m")
        .arg("Closed for test");
    cmd.assert().success();

    // Reopen the issue
    let mut cmd = Command::cargo_bin("git-issue").unwrap();
    cmd.current_dir(&temp)
        .arg("reopen")
        .arg("001")
        .arg("-m")
        .arg("Reopened for test");
    cmd.assert().success().stdout(predicate::str::contains("<<< Reopened for test"));

    // Try to reopen an open issue (should error)
    let mut cmd = Command::cargo_bin("git-issue").unwrap();
    cmd.current_dir(&temp)
        .arg("reopen")
        .arg("001")
        .arg("-m")
        .arg("Should fail");
    cmd.assert().failure().stderr(predicate::str::contains("error: issue is not closed"));
}

#[test]
fn test_list_filters_and_audit_log() {
    let temp = setup_temp_dir();
    let issues_dir = temp.path().join(".issues");
    fs::create_dir_all(&issues_dir).unwrap();

    // Create issues with different labels and states
    let mut cmd = Command::cargo_bin("git-issue").unwrap();
    cmd.current_dir(&temp)
        .arg("create")
        .arg("-t")
        .arg("Bug Issue")
        .arg("-c")
        .arg("Bug details")
        .arg("--label")
        .arg("bug");
    cmd.assert().success();

    let mut cmd = Command::cargo_bin("git-issue").unwrap();
    cmd.current_dir(&temp)
        .arg("create")
        .arg("-t")
        .arg("Enhancement Issue")
        .arg("-c")
        .arg("Enhancement details")
        .arg("--label")
        .arg("enhancement");
    cmd.assert().success();

    // Close the first issue
    let mut cmd = Command::cargo_bin("git-issue").unwrap();
    cmd.current_dir(&temp)
        .arg("close")
        .arg("001")
        .arg("-m")
        .arg("Bug fixed");
    cmd.assert().success();

    // List only open issues
    let mut cmd = Command::cargo_bin("git-issue").unwrap();
    cmd.current_dir(&temp)
        .arg("ls")
        .arg("--state")
        .arg("open");
    cmd.assert().success()
        .stdout(predicate::str::contains("Enhancement Issue"))
        .stdout(predicate::str::contains("enhancement"))
        .stdout(predicate::str::contains("002"));

    // List only closed issues
    let mut cmd = Command::cargo_bin("git-issue").unwrap();
    cmd.current_dir(&temp)
        .arg("ls")
        .arg("--state")
        .arg("closed");
    cmd.assert().success()
        .stdout(predicate::str::contains("Bug Issue"))
        .stdout(predicate::str::contains("[closed]"));

    // List by label
    let mut cmd = Command::cargo_bin("git-issue").unwrap();
    cmd.current_dir(&temp)
        .arg("ls")
        .arg("--label")
        .arg("bug")
        .arg("--state")
        .arg("all");
    cmd.assert().success().stdout(predicate::str::contains("Bug Issue"));

    // Show audit log
    let mut cmd = Command::cargo_bin("git-issue").unwrap();
    cmd.current_dir(&temp)
        .arg("log");
    cmd.assert().success().stdout(predicate::str::contains("CREATE"))
        .stdout(predicate::str::contains("CLOSE"));
}

#[test]
fn test_create_and_list_issue() {
    let temp = setup_temp_dir();
    let issues_dir = temp.path().join(".issues");
    fs::create_dir_all(&issues_dir).unwrap();

    // Create an issue
    let mut cmd = Command::cargo_bin("git-issue").unwrap();
    cmd.current_dir(&temp)
        .arg("create")
        .arg("-t")
        .arg("Test Issue")
        .arg("-c")
        .arg("This is a test issue")
        .arg("--label")
        .arg("bug,ui");
    cmd.assert().success().stdout(predicate::str::contains("Test Issue"));

    // List issues
    let mut cmd = Command::cargo_bin("git-issue").unwrap();
    cmd.current_dir(&temp).arg("ls");
    cmd.assert().success().stdout(predicate::str::contains("Test Issue"));
}

#[test]
fn test_create_subissue_and_close() {
    let temp = setup_temp_dir();
    let issues_dir = temp.path().join(".issues");
    fs::create_dir_all(&issues_dir).unwrap();

    // Create parent issue
    let mut cmd = Command::cargo_bin("git-issue").unwrap();
    cmd.current_dir(&temp)
        .arg("create")
        .arg("-t")
        .arg("Parent")
        .arg("-c")
        .arg("Parent issue");
    cmd.assert().success();

    // Create sub-issue
    let mut cmd = Command::cargo_bin("git-issue").unwrap();
    cmd.current_dir(&temp)
        .arg("create")
        .arg("-p")
        .arg("001")
        .arg("-t")
        .arg("Child")
        .arg("-c")
        .arg("Child issue");
    cmd.assert().success();

    // Try to close parent (should fail, child open)
    let mut cmd = Command::cargo_bin("git-issue").unwrap();
    cmd.current_dir(&temp)
        .arg("close")
        .arg("001")
        .arg("-m")
        .arg("Done");
    cmd.assert().failure().stderr(predicate::str::contains("child issues are still pending"));

    // Close child
    let mut cmd = Command::cargo_bin("git-issue").unwrap();
    cmd.current_dir(&temp)
        .arg("close")
        .arg("001-001")
        .arg("-m")
        .arg("Child done");
    cmd.assert().success();

    // Now close parent
    let mut cmd = Command::cargo_bin("git-issue").unwrap();
    cmd.current_dir(&temp)
        .arg("close")
        .arg("001")
        .arg("-m")
        .arg("Parent done");
    cmd.assert().success();
}
