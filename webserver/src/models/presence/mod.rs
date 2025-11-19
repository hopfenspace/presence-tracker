use galvyn::core::re_exports::time::OffsetDateTime;
use galvyn::rorm;
use galvyn::rorm::and;
use galvyn::rorm::db::Executor;
use galvyn::rorm::fields::types::MaxStr;
use time_tz::OffsetDateTimeExt;
use time_tz::timezones::db::europe;
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

pub struct HistoricPresence {
    pub monday: [f32; 24],
    pub tuesday: [f32; 24],
    pub wednesday: [f32; 24],
    pub thursday: [f32; 24],
    pub friday: [f32; 24],
    pub saturday: [f32; 24],
    pub sunday: [f32; 24],
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

    #[instrument(name = "Presence::historic_presence", skip(exe))]
    pub async fn historic_presence(
        exe: impl Executor<'_>,
        location: Location,
        since: OffsetDateTime,
    ) -> anyhow::Result<HistoricPresence> {
        let mut guard = exe.ensure_transaction().await?;

        let raw_data = rorm::query(guard.get_transaction(), PresenceData)
            .condition(and![
                PresenceData.date_time.greater_than(since),
                PresenceData
                    .location
                    .equals(MaxStr::new(location.to_string())?),
            ])
            .all()
            .await?;

        guard.commit().await?;

        let mut data = vec![vec![vec![]; 24]; 7];

        for point in raw_data {
            // Assumptions were made …
            let local = point.date_time.to_timezone(europe::BERLIN);

            let weekday = local.weekday().number_days_from_monday();
            data.get_mut(weekday as usize)
                .unwrap()
                .get_mut(local.hour() as usize)
                .unwrap()
                .push(point.count);
        }

        let mut historic = HistoricPresence {
            monday: [0.0; 24],
            tuesday: [0.0; 24],
            wednesday: [0.0; 24],
            thursday: [0.0; 24],
            friday: [0.0; 24],
            saturday: [0.0; 24],
            sunday: [0.0; 24],
        };

        for (day, day_data) in data.into_iter().enumerate() {
            for (hour, hour_data) in day_data.into_iter().enumerate() {
                let avg = if hour_data.is_empty() {
                    0.0
                } else {
                    hour_data.iter().sum::<i64>() as f32 / hour_data.len() as f32
                };

                match day {
                    0 => historic.monday[hour] = avg,
                    1 => historic.tuesday[hour] = avg,
                    2 => historic.wednesday[hour] = avg,
                    3 => historic.thursday[hour] = avg,
                    4 => historic.friday[hour] = avg,
                    5 => historic.saturday[hour] = avg,
                    6 => historic.sunday[hour] = avg,
                    _ => unreachable!(),
                }
            }
        }

        Ok(historic)
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
