use serde::{Serialize, Deserialize};
use diesel::prelude::*;
use crate::diesel_schema::osb_user_custom_config;
use diesel::result::Error;
use crate::diesel_schema::osb_github_logfiles::dsl::*;
use crate::diesel_schema::osb_user_custom_config::dsl::*;
use diesel::dsl::exists;
use diesel::dsl::select;

use serde_json::Value;
use chrono::NaiveDateTime;
use bson::Uuid;
use dotenvy::dotenv;
use std::env;
use crate::logfile_model::NewLogfile;
use diesel_async::{RunQueryDsl, AsyncConnection, AsyncPgConnection};

pub async fn get_connection_async() -> Result<AsyncPgConnection, Box<dyn std::error::Error>> {
    dotenv().ok();

    let database_url = env::var("PG_DATABASE_URL").expect("PG_DATABASE_URL must be set");
    let connection = AsyncPgConnection::establish(&database_url).await?;
    Ok(connection)
}
pub async fn get_repos(connection: &mut AsyncPgConnection, limit: i64, offset: i64) -> Result<Vec<String>, Error> {
    let limit = if limit > 0 { limit } else { 50 };

    match osb_user_custom_config
        .select(repo_name)
        .distinct()
        .limit(limit)
        .offset(offset)
        .load::<String>(connection)
        .await {
            Ok(repos) => Ok(repos),
            Err(e) => {
                eprintln!("Error loading repos: offset {} limit {} error {}", limit, offset, e);
                Err(e)
            }
        }
}

pub async fn insert_repos(connection: &mut AsyncPgConnection, repo: NewLogfile) -> Result<String, Error> {
    let rows_inserted = diesel::insert_into(osb_github_logfiles)
        .values(&repo)
        .execute(connection)
        .await?;

    if rows_inserted == 1 {
        Ok(repo.org_repo)
    } else {
        Err(Error::NotFound)
    }
}

#[derive(QueryableByName)]
struct ExistsResult {
    #[sql_type = "diesel::sql_types::Bool"]
    exists: bool,
}

pub async fn verify_tables_exist(connection: &mut AsyncPgConnection) -> Result<(), Error> {
    let logfiles_exists = diesel::sql_query("SELECT EXISTS (SELECT 1 FROM information_schema.tables WHERE table_name = 'osb_github_logfiles')")
        .get_result::<ExistsResult>(connection)
        .await?
        .exists;

    let osb_user_custom_config_exists = diesel::sql_query("SELECT EXISTS (SELECT 1 FROM information_schema.tables WHERE table_name = 'osb_user_custom_config')")
        .get_result::<ExistsResult>(connection)
        .await?
        .exists;

    if logfiles_exists && osb_user_custom_config_exists {
        Ok(())
    } else {
        Err(Error::NotFound)
    }
}