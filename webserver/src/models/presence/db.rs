use galvyn::core::re_exports::time;
use galvyn::core::re_exports::uuid::Uuid;
use galvyn::rorm::Model;
use galvyn::rorm::prelude::ForeignModel;

use crate::models::location::db::LocationModel;

#[derive(Model)]
pub struct PresenceData {
    #[rorm(primary_key)]
    pub uuid: Uuid,

    #[rorm(index)]
    pub date_time: time::OffsetDateTime,

    #[rorm(on_update = "Cascade", on_delete = "Cascade")]
    pub location: ForeignModel<LocationModel>,

    pub count: i64,
}
