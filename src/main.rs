pub mod linear;

use clap::Parser;
use inquire::Select;

#[derive(Parser)]
struct Cli {
    pattern: Option<String>,
    path: Option<std::path::PathBuf>,
}

fn main() {
    let args = Cli::parse();

    // Check if both pattern and path are None, indicating no arguments were provided
    if args.pattern.is_none() && args.path.is_none() {
        selection_menu();
    } else {
        if let Some(pattern) = &args.pattern {
            println!("Pattern: {:?}", pattern);
        } else {
            println!("No pattern provided.");
        }

        if let Some(path) = &args.path {
            println!("Path: {:?}", path);
        } else {
            println!("No path provided.");
        }
    }
}

/// Create an issue for the given team.
fn create_issue_menu() {
    let options = vec!["Create an Issue", "View Your Issues"];
}

fn view_issues() -> String {
    return std::string::String::from("Viewing issues.");
}

/// Creates a menu for the user to select an option from.
fn selection_menu() {
    let options = vec!["Create an Issue", "View Your Issues"];
    let select = Select::new("Choose an option", options);

    match select.prompt() {
        Ok(selected) => match selected {
            "Create an Issue" => {
                println!("Creating an issue...");
                let issue = create_issue_menu();
            }
            "View Your Issues" => {
                println!("Viewing your issues...");
            }
            _ => println!("Unknown option selected."),
        },
        Err(_) => println!("An error occurred while selecting an option."),
    }
}
