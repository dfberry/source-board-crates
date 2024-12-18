use diesel::pg::PgConnection;
//use diesel::sql_types::Jsonb;
use serde::{Serialize, Deserialize};
use diesel::prelude::*;
use crate::diesel_schema::osb_user_custom_config;
use crate::diesel_schema::osb_github_logfiles;
use diesel::result::Error;
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, QueryableByName)]
#[diesel(table_name = osb_user_custom_config)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Watch {
    pub id: String,
    pub user_id: String,
    pub repo_name: String,
    pub created_at: chrono::NaiveDateTime
}

#[derive(Insertable)]
#[diesel(table_name = osb_github_logfiles)]
pub struct NewLogfile{
    #[diesel(sql_type = Jsonb)]
    pub logfile: Value,
    pub org_repo: String
}

pub fn get_connection(database_url: &str) -> PgConnection {

    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub async fn get_repos(connection: &mut PgConnection, limit: i64, offset: i64) -> Result<Vec<String>, Error> {
    use crate::diesel_schema::osb_user_custom_config::dsl::*;

    let limit = if limit > 0 { limit } else { 50 };

    match osb_user_custom_config
        .select(repo_name)
        .distinct()
        .limit(limit)
        .offset(offset)
        .load::<String>(connection) {
            Ok(repos) => Ok(repos),
            Err(e) => {
                eprintln!("Error loading repos: offset {} limit {} error {}", limit, offset, e);
                Err(e)
            }
        }
}
pub async fn insert_repos(connection: &mut PgConnection, repo: NewLogfile) -> Result<String, Error> {
    use crate::diesel_schema::osb_github_logfiles::dsl::*;
    let rows_inserted = diesel::insert_into(osb_github_logfiles)
        .values(&repo)
        .execute(connection)?;

    if rows_inserted == 1 {
        Ok(repo.org_repo)
    } else {
        Err(Error::NotFound)
    }
}