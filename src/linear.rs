use graphql_client::{GraphQLQuery, QueryBody, Response};
use log::log_enabled;
use std::{env, fmt};

const API_ROOT: &str = "https://api.linear.app/graphql";

#[derive(Debug)]
pub struct Team {
    pub id: String,
    pub name: String,
}

impl fmt::Display for Team {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

struct User {
    id: String,
    name: String,
    is_me: bool,
}

struct Issue {
    id: String,
    title: String,
    description: String,
    team: Team,
    creator: User,
    assignee: User,
    status: String,
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
    pub async fn create_issue(&self, name: String, points: i64, team: &Team) {
        let req_body = IssueCreate::build_query(issue_create::Variables {
            title: name,
            points,
            team_id: team.id.clone(),
        });
        let _ = self.make_request(req_body).await;
    }
}
