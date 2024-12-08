use clap::Parser;
use dotenvy::dotenv;
use reqwest::Client;
use serde_json::json;
use serde::{Serialize, Deserialize};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// GitHub organization
    #[clap(short, long)]
    org: String,

    /// GitHub repository
    #[clap(short, long)]
    repo: String,

    /// Personal Access Token
    #[clap(short, long)]
    pat: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct FlattenedRepoData {
    name: String,
    description: Option<String>,
    url: String,
    createdAt: String,
    updatedAt: String,
    pushedAt: String,
    diskUsage: i32,
    watchersCount: i32,
}

impl FlattenedRepoData {
    fn from(repo: Repository) -> Self {

        FlattenedRepoData {
            name: repo.name,
            description: repo.description,
            url: repo.url,
            createdAt: repo.createdAt,
            updatedAt: repo.updatedAt,
            pushedAt: repo.pushedAt,
            diskUsage: repo.diskUsage,
            watchersCount: repo.watchersCount.totalCount,
        }
    }
    fn toJson(&self) -> String {
        json!(self).to_string()
    }
}

#[derive(Deserialize, Debug)]
struct Repository {
    name: String,
    description: Option<String>,
    url: String,
    createdAt: String,
    updatedAt: String,
    pushedAt: String,
    diskUsage: i32,
    #[serde(rename = "watchers")]
    watchersCount: WatchersCount,
}

#[derive(Deserialize, Debug)]
struct WatchersCount {
    totalCount: i32,
}

#[derive(Deserialize, Debug)]
struct RepoData {
    repository: Repository,
}

#[derive(Deserialize, Debug)]
struct OrgData {
    organization: RepoData,
}

#[derive(Deserialize, Debug)]
struct ResponseData {
    data: OrgData,
}

async fn query_github_api(
    org: &str, 
    repo: &str, 
    pat: &str
) -> Result<ResponseData, Box<dyn std::error::Error>> {
    let client = Client::new();
    let query = r#"
        query RepoDetails($organization: String!, $repository: String!) {
            organization(login: $organization) {
                repository(name: $repository) {
                    name
                    description
                    url
                    createdAt
                    updatedAt
                    pushedAt
                    diskUsage
                    watchers {
                        totalCount
                    }
                }
            }
        }
    "#;

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