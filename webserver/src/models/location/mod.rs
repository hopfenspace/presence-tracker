use galvyn::core::re_exports::schemars;
use galvyn::core::re_exports::schemars::JsonSchema;
use galvyn::rorm;
use galvyn::rorm::db::Executor;
use galvyn::rorm::fields::types::MaxStr;
use serde::Deserialize;
use serde::Serialize;
use strum::IntoEnumIterator;

use crate::models::location::db::LocationModel;

pub(in crate::models) mod db;

#[derive(
    Debug, Clone, Copy, Serialize, Deserialize, JsonSchema, strum::Display, strum::EnumIter,
)]
pub enum Location {
    Bunker,
    Utopia,
}

impl Location {
    pub async fn init_db(exe: impl Executor<'_>) -> anyhow::Result<()> {
        let mut guard = exe.ensure_transaction().await?;

        let count = rorm::query(guard.get_transaction(), LocationModel.location.count())
            .one()
            .await?;

        if count == 0 {
            rorm::insert(guard.get_transaction(), LocationModel)
                .bulk(Location::iter().map(|x| LocationModel {
                    location: MaxStr::new(x.to_string()).unwrap(),
                }))
                .await?;
        }

        guard.commit().await?;

        Ok(())
    }
}
