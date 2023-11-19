use graphql_client::{GraphQLQuery, QueryBody, Response};
use std::{env, fmt};

const API_ROOT: &str = "https://api.linear.app/graphql";

pub struct Team {
    id: String,
    name: String,
}

impl fmt::Display for Team {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Team ID: {}, Name: {}", self.id, self.name)
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

pub struct LinearClient {
    api_key: String,
    client: reqwest::Client,
}

impl LinearClient {
    pub fn new() -> Self {
        let api_key = env::var("LINEAR_API_KEY").expect("LINEAR_API_KEY not found in .env file");
        let client = reqwest::Client::new();
        LinearClient { api_key, client }
    }

    /// Makes a request to the Linear API with a given query.
    async fn make_request(
        &self,
        query: QueryBody<teams::Variables>,
    ) -> Result<reqwest::Response, reqwest::Error> {
        let request = self
            .client
            .get(API_ROOT)
            .header("Authorization", format!("Bearer {}", self.api_key))
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
}
