use std::net::IpAddr;
use std::net::Ipv4Addr;
use std::sync::LazyLock;

use galvyn::core::stuff::env::EnvVar;
use galvyn::rorm::DatabaseDriver;

/// Address the API server should bind to
pub static LISTEN_ADDRESS: EnvVar<IpAddr> =
    EnvVar::optional("LISTEN_ADDRESS", || IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)));

/// Port the API server should bind to
pub static LISTEN_PORT: EnvVar<u16> = EnvVar::optional("LISTEN_PORT", || 8080);

/// The address of the database server
pub static POSTGRES_HOST: EnvVar = EnvVar::required("POSTGRES_HOST");

/// The database name
pub static POSTGRES_DB: EnvVar = EnvVar::required("POSTGRES_DB");

/// The port of the database server
pub static POSTGRES_PORT: EnvVar<u16> = EnvVar::required("POSTGRES_PORT");

/// The user to use for the database connection
pub static POSTGRES_USER: EnvVar = EnvVar::required("POSTGRES_USER");

/// Password for the user
pub static POSTGRES_PASSWORD: EnvVar = EnvVar::required("POSTGRES_PASSWORD");

/// Bundle of all database variables combined in `rorm`'s format
pub static DB: LazyLock<DatabaseDriver> = LazyLock::new(|| DatabaseDriver::Postgres {
    name: POSTGRES_DB.clone(),
    host: POSTGRES_HOST.clone(),
    port: *POSTGRES_PORT,
    user: POSTGRES_USER.clone(),
    password: POSTGRES_PASSWORD.clone(),
});
