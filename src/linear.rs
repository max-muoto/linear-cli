use graphql_client::GraphQLQuery;
use std::env;

const API_ROOT: &str = "https://api.linear.app/graphql";

struct Team {
    id: String,
    name: String,
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

struct LinearClient {
    api_key: String,
    client: reqwest::Client,
}

impl LinearClient {
    async fn new() -> Self {
        let api_key = env::var("LINEAR_API_KEY").expect("LINEAR_API_KEY not found in .env file");
        let client = reqwest::Client::new();
        LinearClient { api_key, client }
    }

    /// Makes a request to the Linear API with a given query.
    async fn make_request(
        &self,
        query: graphql_client::QueryBody<Any>,
    ) -> Result<reqwest::Response, reqwest::Error> {
        let request = self
            .client
            .post(API_ROOT)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&query)
            .send()
            .await?;

        Ok(request)
    }

    /// Gets a list of teams from the Linear API.
    async fn get_teams(&self) {
        let req_body = Teams::build_query(teams::Variables {});
        let mut res = self.make_request(req_body).await?;
        // let response_body: Response<teams::ResponseData> = res.json().await?;
    }
}
