use clap::Parser;
use reqwest::Client;
use serde_json::json;
use serde::{Serialize, Deserialize};

pub mod models;
pub mod query;

use models::{
    Args,
    FlattenedRepoData,
    ResponseData
};
use query::REPO_DETAILS_QUERY;

async fn query_github_api(
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

#[tokio::main]
async fn main() {
    
    let args = Args::parse();

    if args.org.trim().is_empty() || args.repo.trim().is_empty() || args.pat.trim().is_empty() {
        panic!("All three arguments (org, repo, and pat) are required.");
    }

    match query_github_api(&args.org, &args.repo, &args.pat).await {
        Ok(repo_data) => {
            let json_data = FlattenedRepoData::from(repo_data.data.organization.repository).toJson();
            println!("{}", json_data);
        }
        Err(e) => {
            eprintln!("Error querying GitHub API: {}", e);
        }
    }
}