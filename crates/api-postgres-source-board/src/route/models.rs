use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::route::schema::*;

#[derive(Queryable, Identifiable, Serialize, Deserialize, Debug)]
#[table_name = "osb_session"]
pub struct OsbSession {
    pub id: String,
    pub user_id: String,
    pub expires_at: chrono::NaiveDateTime,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[table_name = "osb_session"]
pub struct NewOsbSession<'a> {
    pub id: &'a str,
    pub user_id: &'a str,
    pub expires_at: chrono::NaiveDateTime,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Queryable, Identifiable, Serialize, Deserialize, Debug)]
#[table_name = "osb_token"]
pub struct OsbToken {
    pub id: String,
    pub user_id: String,
    pub access_token: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[table_name = "osb_token"]
pub struct NewOsbToken<'a> {
    pub id: &'a str,
    pub user_id: &'a str,
    pub access_token: &'a str,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Queryable, Identifiable, Serialize, Deserialize, Debug)]
#[table_name = "osb_user"]
pub struct OsbUser {
    pub id: String,
    pub github_id: String,
    pub username: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[table_name = "osb_user"]
pub struct NewOsbUser<'a> {
    pub id: &'a str,
    pub github_id: &'a str,
    pub username: &'a str,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Queryable, Selectable, Identifiable, Serialize, Deserialize, Debug)]
#[table_name = "osb_user_custom_config"]
pub struct OsbUserCustomConfig {
    pub id: String,
    pub user_id: String,
    pub repo_name: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[table_name = "osb_user_custom_config"]
pub struct NewOsbUserCustomConfig<'a> {
    pub id: &'a str,
    pub user_id: &'a str,
    pub repo_name: &'a str,
    pub created_at: chrono::NaiveDateTime,
}