use galvyn::core::Module;
use galvyn::core::re_exports::axum::extract::Query;
use galvyn::core::re_exports::schemars;
use galvyn::core::re_exports::schemars::JsonSchema;
use galvyn::core::re_exports::time::OffsetDateTime;
use galvyn::core::stuff::api_error::ApiError;
use galvyn::core::stuff::api_error::ApiResult;
use galvyn::core::stuff::api_json::ApiJson;
use galvyn::core::stuff::schema::SchemaDateTime;
use galvyn::get;
use galvyn::rorm::Database;
use serde::Deserialize;
use serde::Serialize;

use crate::models::location::Location;
use crate::models::presence::Presence;

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct LocationQuery {
    pub loc: Location,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct CurrentPresence {
    pub current_count: u8,
    pub current_time: SchemaDateTime,
    pub last_vacancy: SchemaDateTime,
}

pub struct WeeklyHistoricPresence {
    pub monday: [u8; 24],
    pub tuesday: [u8; 24],
    pub wednesday: [u8; 24],
    pub thursday: [u8; 24],
    pub friday: [u8; 24],
    pub saturday: [u8; 24],
    pub sunday: [u8; 24],
}

#[get("/current-presence")]
pub async fn current_presence(
    Query(LocationQuery { loc }): Query<LocationQuery>,
) -> ApiResult<ApiJson<CurrentPresence>> {
    let mut tx = Database::global().start_transaction().await?;

    let curr = Presence::current_presence(&mut tx, loc).await?;

    tx.commit().await?;

    Ok(ApiJson(CurrentPresence {
        current_count: curr.current_count,
        current_time: SchemaDateTime(curr.current_time),
        last_vacancy: SchemaDateTime(curr.last_vacancy),
    }))
}
