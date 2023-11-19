pub mod linear;

use clap::{Parser, Subcommand};
use inquire::{Select, Text};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    // Create a Linear issue.
    Create {
        #[arg(short, long)]
        name: String,
        #[arg(short, long)]
        points: i64,
        #[arg(short, long)]
        team: String,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let linear_client = linear::LinearClient::new();

    match &cli.command {
        Some(Commands::Create { name, points, team }) => {
            let teams = linear_client
                .get_teams()
                .await
                .expect("Failed to get teams.");
            let team_result = teams
                .iter()
                .filter(|curr_team| curr_team.name == team.as_str())
                .try_fold(None, |acc, team| match acc {
                    None => Ok(Some(team)),
                    Some(_) => Err("Multiple teams with the same name found".to_string()),
                });
            let selected_team = match team_result {
                Ok(Some(team)) => Ok(team),
                Ok(None) => Err("No team found with the given name".to_string()),
                Err(e) => Err(e),
            };
            linear_client
                .create_issue(name.to_string(), *points, selected_team.unwrap())
                .await;
        }
        None => {
            // Default to the selection menu if no subcommand is given.
            selection_menu(&linear_client).await;
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
                    linear_client
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

/// Creates a menu for the user to select an option from.
async fn selection_menu(linear_client: &linear::LinearClient) {
    let options = vec!["Create an Issue", "View Your Issues"];
    let select = Select::new("Choose an option", options);

    match select.prompt() {
        Ok(selected) => match selected {
            "Create an Issue" => {
                create_issue_menu(&linear_client).await;
            }
            "View Your Issues" => {
                println!("Viewing your issues...");
            }
            _ => println!("Unknown option selected."),
        },
        Err(_) => println!("An error occurred while selecting an option."),
    }
}
