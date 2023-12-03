//! This module contains formatting utilities for the CLI tool.

use crate::linear::Issue;

use tabled::{settings::Style, Table, Tabled};

// Format a string as a hyperlink.
fn format_as_hyperlink(url: &str, text: &str) -> String {
    format!("\x1B]8;;{}\x1B\\{}\x1B]8;;\x1B\\", url, text)
}

#[derive(Tabled)]
struct IssueDisplay {
    #[tabled(rename = "Issue")]
    link: String,

    #[tabled(rename = "Team")]
    team_name: String,

    #[tabled(rename = "State")]
    state_name: String,
}

impl From<Issue> for IssueDisplay {
    fn from(issue: Issue) -> Self {
        IssueDisplay {
            team_name: issue.team_name,
            state_name: issue.state_name,
            link: format_as_hyperlink(&issue.url, &issue.title),
        }
    }
}

/// Print a list of issues in a table.
pub fn print_issues(issues: Vec<Issue>) {
    let issues_display: Vec<IssueDisplay> = issues.into_iter().map(IssueDisplay::from).collect();

    let mut binding = Table::new(&issues_display);
    let table = binding.with(Style::ascii());

    println!("{}", table.to_string());
}
