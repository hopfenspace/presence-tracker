use galvyn::core::InitError;
use galvyn::core::Module;
use galvyn::core::PreInitError;
use galvyn::rorm::Database;

use crate::models::location::Location;

pub struct DbInit;

impl Module for DbInit {
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

        Location::init_db(&mut tx).await?;

        tx.commit().await?;

        Ok(DbInit)
    }
}
