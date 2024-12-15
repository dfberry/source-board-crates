use reqwest::Client;
use crate::models::ResponseData;
use crate::graphql_query::REPO_DETAILS_QUERY;

pub async fn query_github_api(
    org: &str, 
    repo: &str, 
    pat: &str
) -> Result<ResponseData, Box<dyn std::error::Error>> {
    let client = Client::new();
    let query = REPO_DETAILS_QUERY;

    let variables = serde_json::json!({
        "organization": org,
        "repository": repo,
    });

    let response = client
        .post("https://api.github.com/graphql")
        .header("Authorization", format!("Bearer {}", pat))
        .header("User-Agent", "rust-github-client")
        .json(&serde_json::json!({
            "query": query,
            "variables": variables,
        }))
        .send()
        .await?
        .json::<ResponseData>()
        .await?;

    Ok(response)
}