use galvyn::core::re_exports::time::OffsetDateTime;
use galvyn::rorm;
use galvyn::rorm::and;
use galvyn::rorm::db::Executor;
use galvyn::rorm::fields::types::MaxStr;
use tracing::instrument;

use crate::models::location::Location;
use crate::models::presence::db::PresenceData;

pub mod db;

pub struct Presence;

pub struct CurrentPresence {
    pub current_count: u8,
    pub current_time: OffsetDateTime,
    pub last_vacancy: OffsetDateTime,
}

impl Presence {
    #[instrument(name = "Presence::current_presence", skip(exe))]
    pub async fn current_presence(
        exe: impl Executor<'_>,
        location: Location,
    ) -> anyhow::Result<CurrentPresence> {
        let mut guard = exe.ensure_transaction().await?;

        let location = MaxStr::new(location.to_string())?;

        let current_presence_data = rorm::query(guard.get_transaction(), PresenceData)
            .condition(PresenceData.location.equals(location.clone()))
            .order_desc(PresenceData.date_time)
            .optional()
            .await?
            .ok_or(anyhow::anyhow!("No presence data found"))?;

        let last_vacancy = rorm::query(guard.get_transaction(), PresenceData.date_time)
            .condition(and![
                PresenceData.count.equals(0),
                PresenceData.location.equals(location.clone()),
            ])
            .order_desc(PresenceData.date_time)
            .one()
            .await?;

        guard.commit().await?;

        Ok(CurrentPresence {
            current_count: current_presence_data.count as u8,
            current_time: current_presence_data.date_time,
            last_vacancy,
        })
    }

    #[instrument(name = "Presence::count_data", skip(exe))]
    pub async fn count_data(exe: impl Executor<'_>) -> anyhow::Result<u64> {
        Ok(rorm::query(exe, PresenceData.uuid.count()).one().await? as u64)
    }

    #[instrument(name = "Presence::insert_bulk", skip(exe, data))]
    pub async fn insert_bulk(exe: impl Executor<'_>, data: &[PresenceData]) -> anyhow::Result<()> {
        rorm::insert(exe, PresenceData).bulk(data).await?;

        Ok(())
    }
}
