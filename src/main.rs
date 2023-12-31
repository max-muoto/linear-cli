pub mod formatting;
pub mod linear;

use clap::{Parser, Subcommand};
use clipboard::{ClipboardContext, ClipboardProvider};
use inquire::{MultiSelect, Select, Text};

use crate::formatting::print_issues;

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
        #[arg(short, long)]
        copy: bool,
    },
    // View assigned issues.
    View,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let linear_client = linear::LinearClient::new();
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();

    match &cli.command {
        Some(Commands::Create {
            name,
            points,
            team,
            copy,
        }) => {
            let teams = linear_client
                .get_teams()
                .await
                .expect("Failed to get teams.");
            let team = teams
                .iter()
                .filter(|curr_team| curr_team.name == team.as_str())
                .try_fold(None, |acc, team| match acc {
                    None => Ok(Some(team)),
                    Some(_) => Err("Multiple teams with the same name found".to_string()),
                });
            match team {
                Ok(Some(team)) => {
                    println!("Creating issue...");
                    let url = linear_client
                        .create_issue(name.to_string(), *points, &team)
                        .await;
                    if *copy {
                        println!("Coped URL to clipboard.");
                        let _ = ctx.set_contents(url.to_owned());
                    }
                }
                Ok(None) => println!("No team with the given name found."),
                Err(err) => println!("{}", err),
            }
        }
        Some(Commands::View {}) => {
            view_issues_menu(&linear_client).await;
        }
        None => {
            // Default to the selection menu if no subcommand is given.
            selection_menu(&linear_client).await;
        }
    }
}

/// Menu to create an issue.
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

/// Menu to view issues for all temas.
async fn view_issues_menu(linear_client: &linear::LinearClient) {
    let teams = linear_client
        .get_teams()
        .await
        .expect("Failed to get teams.");

    let select_team = Select::new("Choose a team", teams);

    match select_team.prompt() {
        Ok(selected_team) => {
            let workflow_states = linear_client
                .get_workflows_states(Some(selected_team.id.clone()))
                .await
                .expect("Failed to get workflow states.");
            let select_workflow_state =
                MultiSelect::new("Choose a workflow state", workflow_states);
            match select_workflow_state.prompt() {
                Ok(selected_workflow_states) => {
                    println!("Viewing issues...");
                    let issues = linear_client
                        .get_assigned_issues(
                            Some(selected_team.id.clone()),
                            Some(selected_workflow_states),
                        )
                        .await
                        .expect("Failed to get issues.");
                    print_issues(issues);
                }
                Err(_) => println!("An error occurred while selecting a workflow state."),
            }
        }
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
                view_issues_menu(&linear_client).await;
            }
            _ => println!("Unknown option selected."),
        },
        Err(_) => println!("An error occurred while selecting an option."),
    }
}
