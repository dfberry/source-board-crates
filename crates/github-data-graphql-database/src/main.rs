use serde_json::json;
use dotenvy::dotenv;
use std::env;
use chrono::Utc;

pub mod models;
pub mod graphql_query;
pub mod github_query;
pub mod mongo_query;
pub mod pq_query;
pub mod diesel_schema;

use models::FlattenedRepoData;
use github_query::query_github_api;
use mongo_query::create;


#[tokio::main]
async fn main() {
    
    dotenv().ok();

    let pat = env::var("PAT").expect("PAT environment variable not set");
    let pg_database_url = env::var("PG_DATABASE_URL").expect("PG_DATABASE_URL environment variable not set");
    let mongo_database_url = env::var("MONGO_DATABASE_URL").expect("MONGO_DATABASE_URL environment variable not set");

    let mut pg_connection = pq_query::get_connection(&pg_database_url);
    let mut pg_offset = 0;
    let pg_limit = 50;
    let mut pg_end: bool = false;

    let mongo_connection = mongo_query::get_connection(mongo_database_url).await;
    let database = mongo_connection.database("osb");
    let collection = database.collection::<bson::Document>("repos_historical_data");

    loop {
        // Perform the operations you want to repeat
        let list_of_repos = pq_query::get_repos(&mut pg_connection, pg_limit, pg_offset).await;
        println!("Count of repos: {:?}", list_of_repos);

        // if length of list_of_repos is less than limit, then we have reached the end
        if list_of_repos.len() < pg_limit as usize {
            pg_end = true;
        } else {
            // Update the offset for the next iteration
            pg_offset += pg_limit;
        }

        // Process the list_of_repos
        for repo in list_of_repos {
            println!("{}\n", repo);

            // break the string into two parts using the '/' character
            let parts: Vec<&str> = repo.split('/').collect();
            //println!("Parts: {:?}", parts);

            let github_data = query_github_api(&parts[0], &parts[1], &pat).await.unwrap();    
            //println!("GitHub Data: {:?}", github_data);      

            // flatten data
            let flattened_data = FlattenedRepoData::from(github_data.data.organization.repository);
            let mut json_data = json!(flattened_data);
            println!("{}\n", json_data);

            // add parts[0] as org, parts[1] as repo, and current time to the json data
            json_data["org"] = json!(parts[0]);
            json_data["repo"] = json!(parts[1]);
            json_data["log_time"] = json!(Utc::now().to_rfc3339());

            // insert the json_data into the mongo database
            create(&collection, json_data).await.unwrap();

        }
    
        // Check the condition to break the loop
        if pg_end {
            break;
        }
    }


    // let list_of_repos = pqquery::get_repos(&mut connection, 50, 0).await;

    // match query_github_api(&args.org, &args.repo, &args.pat).await {
    //     Ok(repo_data) => {
    //         let json_data = FlattenedRepoData::from(repo_data.data.organization.repository).toJson();
    //         println!("{}", json_data);
    //     }
    //     Err(e) => {
    //         eprintln!("Error querying GitHub API: {}", e);
    //     }
    // }
}