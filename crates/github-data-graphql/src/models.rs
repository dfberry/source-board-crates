use clap::Parser;
use serde_json::json;
use serde::{Serialize, Deserialize};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// GitHub organization
    #[clap(short, long)]
    pub org: String,

    /// GitHub repository
    #[clap(short, long)]
    pub repo: String,

    /// Personal Access Token
    #[clap(short, long)]
    pub pat: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct FlattenedRepoData {
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
    pub fn from(repo: Repository) -> Self {

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
    pub fn toJson(&self) -> String {
        json!(self).to_string()
    }
}

#[derive(Deserialize, Debug)]
pub struct Repository {
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
pub struct WatchersCount {
    totalCount: i32,
}

#[derive(Deserialize, Debug)]
pub struct RepoData {
    pub repository: Repository,
}

#[derive(Deserialize, Debug)]
pub struct OrgData {
    pub organization: RepoData,
}

#[derive(Deserialize, Debug)]
pub struct ResponseData {
    pub data: OrgData,
}