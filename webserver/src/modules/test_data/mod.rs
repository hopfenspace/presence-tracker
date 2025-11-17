use std::time::Duration;

use galvyn::core::InitError;
use galvyn::core::Module;
use galvyn::core::PreInitError;
use galvyn::core::re_exports::time::OffsetDateTime;
use galvyn::core::re_exports::uuid::Uuid;
use galvyn::rorm::Database;
use galvyn::rorm::fields::types::MaxStr;
use galvyn::rorm::prelude::ForeignModelByField;
use strum::IntoEnumIterator;
use tracing::info;
use tracing::warn;

use crate::models::location::Location;
use crate::models::presence::Presence;
use crate::models::presence::db::PresenceData;

pub struct TestData;

impl Module for TestData {
    type Setup = ();
    type PreInit = ();

    async fn pre_init(_setup: Self::Setup) -> Result<Self::PreInit, PreInitError> {
        Ok(())
    }

    type Dependencies = (Database,);

    async fn init(
        _pre_init: Self::PreInit,
        dependencies: &mut Self::Dependencies,
    ) -> Result<Self, InitError> {
        let mut tx = dependencies.0.start_transaction().await?;

        let count = Presence::count_data(&mut tx).await?;
        if count > 0 {
            warn!("Skipping test_data data generation, already present");
            return Ok(TestData);
        }

        info!("Generating test data");

        let mut data = Vec::with_capacity(20 * 24 * 7 * 4);
        let last = OffsetDateTime::now_utc() - Duration::from_hours(24 * 7 * 4);
        let mut curr = OffsetDateTime::now_utc();
        loop {
            if curr < last {
                break;
            }

            for location in Location::iter() {
                data.push(PresenceData {
                    uuid: Uuid::new_v4(),
                    date_time: curr,
                    location: ForeignModelByField(MaxStr::new(location.to_string())?),
                    count: rand::random_range(0..16),
                });
            }

            curr -= Duration::from_mins(5);
        }

        Presence::insert_bulk(&mut tx, &data).await?;

        tx.commit().await?;

        Ok(TestData)
    }
}
