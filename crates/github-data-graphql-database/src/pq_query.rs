use diesel::pg::PgConnection;
use serde::{Serialize, Deserialize};
use diesel::prelude::*;
use crate::diesel_schema::osb_user_custom_config;

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, QueryableByName)]
#[diesel(table_name = osb_user_custom_config)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Watch {
    pub id: String,
    pub user_id: String,
    pub repo_name: String,
    pub created_at: chrono::NaiveDateTime
}

pub fn get_connection(database_url: &str) -> PgConnection {

    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub async fn get_repos(connection: &mut PgConnection, limit: i64, offset: i64) -> Vec<String> {
    use crate::diesel_schema::osb_user_custom_config::dsl::*;

    let limit = if limit > 0 { limit } else { 50 };

    osb_user_custom_config
        .select(repo_name)
        .distinct()
        .limit(limit)
        .offset(offset)
        .load(connection)
        .expect("Error loading users")
}