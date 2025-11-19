use galvyn::core::Module;
use galvyn::core::re_exports::axum::extract::Query;
use galvyn::core::re_exports::schemars;
use galvyn::core::re_exports::schemars::JsonSchema;
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
pub struct GetDataQuery {
    pub loc: Location,
    pub since: SchemaDateTime,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct PresenceData {
    pub current_count: u8,
    pub current_time: SchemaDateTime,
    pub last_vacancy: SchemaDateTime,
    pub history: WeeklyHistoricPresence,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct WeeklyHistoricPresence {
    pub monday: [f32; 24],
    pub tuesday: [f32; 24],
    pub wednesday: [f32; 24],
    pub thursday: [f32; 24],
    pub friday: [f32; 24],
    pub saturday: [f32; 24],
    pub sunday: [f32; 24],
}

#[get("/data")]
pub async fn data(
    Query(GetDataQuery { loc, since }): Query<GetDataQuery>,
) -> ApiResult<ApiJson<PresenceData>> {
    let mut tx = Database::global().start_transaction().await?;

    let curr = Presence::current_presence(&mut tx, loc).await?;
    let historic = Presence::historic_presence(&mut tx, loc, since.0).await?;

    tx.commit().await?;

    Ok(ApiJson(PresenceData {
        current_count: curr.current_count,
        current_time: SchemaDateTime(curr.current_time),
        last_vacancy: SchemaDateTime(curr.last_vacancy),
        history: WeeklyHistoricPresence {
            monday: historic.monday,
            tuesday: historic.tuesday,
            wednesday: historic.wednesday,
            thursday: historic.thursday,
            friday: historic.friday,
            saturday: historic.saturday,
            sunday: historic.sunday,
        },
    }))
}
