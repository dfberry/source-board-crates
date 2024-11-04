use diesel::prelude::*;
use crate::State;
use crate::Json;
use crate::StatusCode;
use crate::route::error::internal_error;
use crate::route::models::*;
use crate::route::schema::*;

pub async fn listUsersConfigs(
    State(pool): State<deadpool_diesel::postgres::Pool>,
) -> Result<Json<Vec<OsbUserCustomConfig>>, (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_error)?;
    let res = conn
        .interact(|conn| osb_user_custom_config::table.select(OsbUserCustomConfig::as_select()).load(conn))
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;
    Ok(Json(res))
}

pub async fn listGitHubUsers(
    State(pool): State<deadpool_diesel::postgres::Pool>,
) -> Result<Json<Vec<OsbUser>>, (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_error)?;
    let res = conn
        .interact(|conn| osb_user::table.select(OsbUser::as_select()).load(conn))
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;
    Ok(Json(res))
}