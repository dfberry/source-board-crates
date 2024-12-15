use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::result::Error;
use diesel::Queryable;
use diesel::Insertable;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::NaiveDateTime;
use serde_json::Value;
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