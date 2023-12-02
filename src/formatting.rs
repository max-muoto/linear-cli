//! This module contains formatting utilities for the CLI tool.
use prettytable::{row, Cell, Row, Table};

use crate::linear::Issue;

/// Print a list of issues in a table.
pub fn print_issues(issues: Vec<Issue>) {
    let mut table = Table::new();
    table.add_row(row!["Issue", "Team"]);
    for issue in issues {
        table.add_row(Row::new(vec![
            Cell::new(&issue.title),
            Cell::new(&issue.team.name),
        ]));
    }
    table.printstd();
}
