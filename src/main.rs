pub mod linear;

use clap::Parser;
use inquire::{Select, Text};

#[derive(Parser)]
struct Cli {
    pattern: Option<String>,
    path: Option<std::path::PathBuf>,
}

#[tokio::main]
async fn main() {
    env_logger::init();
    let args = Cli::parse();
    let linear_client = linear::LinearClient::new();

    // Check if both pattern and path are None, indicating no arguments were provided
    if args.pattern.is_none() && args.path.is_none() {
        selection_menu(&linear_client).await;
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
async fn create_issue_menu(linear_client: &linear::LinearClient) {
    let teams = linear_client
        .get_teams()
        .await
        .expect("Failed to get teams.");

    let select_team = Select::new("Choose a team", teams);
    let select_points = Select::new("Choose a point value", vec![1, 2, 3, 5, 8]);
    let name = Text::new("What is the name of the issue?").prompt();

    match select_team.prompt() {
        Ok(selected_team) => match select_points.prompt() {
            Ok(selected_points) => match name {
                Ok(issue_name) => {
                    println!("Creating issue...");
                    let issue = linear_client
                        .create_issue(issue_name, selected_points, &selected_team)
                        .await;
                }
                Err(_) => println!("An error occurred while getting the issue name."),
            },
            Err(_) => println!("An error occurred while selecting a point value."),
        },
        Err(_) => println!("An error occurred while selecting a team."),
    }
}

async fn view_issues() -> String {
    return std::string::String::from("Viewing issues.");
}

/// Creates a menu for the user to select an option from.
async fn selection_menu(linear_client: &linear::LinearClient) {
    let options = vec!["Create an Issue", "View Your Issues"];
    let select = Select::new("Choose an option", options);

    match select.prompt() {
        Ok(selected) => match selected {
            "Create an Issue" => {
                println!("Creating an issue...");
                let issue = create_issue_menu(&linear_client).await;
            }
            "View Your Issues" => {
                println!("Viewing your issues...");
            }
            _ => println!("Unknown option selected."),
        },
        Err(_) => println!("An error occurred while selecting an option."),
    }
}
