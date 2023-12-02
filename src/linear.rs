use graphql_client::{GraphQLQuery, QueryBody, Response};
use log::log_enabled;
use std::{env, fmt};

const API_ROOT: &str = "https://api.linear.app/graphql";

#[derive(Debug)]
pub struct Team {
    pub id: String,
    pub name: String,
}

pub struct WorkflowState {
    pub name: String,
    pub id: String,
    pub team_id: String,
}

impl fmt::Display for Team {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl fmt::Display for WorkflowState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

struct User {
    id: String,
    name: String,
    is_me: bool,
}

pub struct Issue {
    pub id: String,
    pub title: String,
    pub team: Team,
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/schemas/linear_schema.json",
    query_path = "src/queries/teams.graphql"
)]
pub struct Teams;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/schemas/linear_schema.json",
    query_path = "src/queries/create_issue.graphql"
)]
pub struct IssueCreate;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/schemas/linear_schema.json",
    query_path = "src/queries/workflow_states.graphql"
)]
pub struct WorkflowStates;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/schemas/linear_schema.json",
    query_path = "src/queries/current_user.graphql"
)]
pub struct CurrentUser;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/schemas/linear_schema.json",
    query_path = "src/queries/assigned_issues.graphql"
)]
pub struct AssignedIssues;

pub struct LinearClient {
    api_key: String,
    client: reqwest::Client,
}

impl LinearClient {
    pub fn new() -> Self {
        let api_key = env::var("LINEAR_API_KEY").expect("LINEAR_API_KEY not found in .env file");
        let client = reqwest::Client::builder()
            .connection_verbose(log_enabled!(log::Level::Trace))
            .build()
            .expect("Failed to construct client");
        LinearClient { api_key, client }
    }

    /// Makes a request to the Linear API with a given query.
    async fn make_request<V: serde::Serialize>(
        &self,
        query: QueryBody<V>,
    ) -> Result<reqwest::Response, reqwest::Error> {
        let request = self
            .client
            .post(API_ROOT)
            .header("Authorization", &self.api_key)
            .json(&query)
            .send()
            .await?;
        Ok(request)
    }

    /// Gets a list of teams from the Linear API.
    pub async fn get_teams(&self) -> Result<Vec<Team>, Box<dyn std::error::Error>> {
        let req_body = Teams::build_query(teams::Variables {});
        let res = self.make_request(req_body);
        let response_body: Response<teams::ResponseData> = res.await?.json().await?;
        let response_data = response_body.data.expect("No response data found.");

        let teams = response_data
            .teams
            .nodes
            .into_iter()
            .map(|team_data| Team {
                id: team_data.id,
                name: team_data.name,
            })
            .collect();

        Ok(teams)
    }

    /// Creates an issue for the given team.
    pub async fn create_issue(&self, name: String, points: i64, team: &Team) -> String {
        let req_body = IssueCreate::build_query(issue_create::Variables {
            title: name,
            points,
            team_id: team.id.clone(),
        });
        let res = self
            .make_request(req_body)
            .await
            .expect("Failed to create issue.");
        let response_body: Response<issue_create::ResponseData> = res.json().await.unwrap();
        let response_data = response_body.data.expect("No response data found.");
        response_data.issue_create.issue.unwrap().url
    }

    /// Get all workflow states, for a specific team if a `team_id` is provided.
    pub async fn get_workflows_states(
        &self,
        team_id: Option<String>,
    ) -> Result<Vec<WorkflowState>, Box<dyn std::error::Error>> {
        let req_body = WorkflowStates::build_query(workflow_states::Variables {});
        let res = self.make_request(req_body);
        let response_body: Response<workflow_states::ResponseData> = res.await?.json().await?;
        let response_data = response_body.data.expect("No response data found");

        let states = response_data
            .workflow_states
            .nodes
            .into_iter()
            .filter(|workflow_state| match &team_id {
                Some(team_id) => workflow_state.team.id == *team_id,
                None => true,
            })
            .map(|workflow_state| WorkflowState {
                id: workflow_state.id,
                name: workflow_state.name,
                team_id: workflow_state.team.id,
            })
            .collect();

        Ok(states)
    }

    /// Get issues for the currently authenticated user.
    pub async fn get_assigned_issues(&self) -> Result<Vec<Issue>, Box<dyn std::error::Error>> {
        let req_body = AssignedIssues::build_query(assigned_issues::Variables {});
        let res = self.make_request(req_body);
        let response_body: Response<assigned_issues::ResponseData> = res.await?.json().await?;
        let response_data = response_body.data.expect("No response data found");

        let issues = response_data
            .viewer
            .assigned_issues
            .nodes
            .into_iter()
            .map(|issue| {
                let team = Team {
                    name: issue.team.name,
                    id: String::from("4"),
                };
                Issue {
                    id: issue.id,
                    title: issue.title,
                    team,
                }
            })
            .collect();
        Ok(issues)
    }
}
