use once_cell::sync::Lazy;
use serde::Deserialize;
use std::{fs::File, io::Read, path::Path};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct DbConfig {
    /// Settings for the primary database. This is usually writeable, but will be read-only in
    /// some configurations.
    /// An optional follower database. Always read-only.
    pub url: String,
    #[serde(default = "default_db_pooll_size")]
    pub pool_size: u32,
    pub min_idle: Option<u32>,

    /// Number of seconds to wait for unacknowledged TCP packets before treating the connection as
    /// broken. This value will determine how long crates.io stays unavailable in case of full
    /// packet loss between the application and the database: setting it too high will result in an
    /// unnecessarily long outage (before the unhealthy database logic kicks in), while setting it
    /// too low might result in healthy connections being dropped.
    #[serde(default = "default_tcp_timeout")]
    pub tcp_timeout: u64,
    /// Time to wait for a connection to become available from the connection
    /// pool before returning an error.
    /// Time to wait for a connection to become available from the connection
    /// pool before returning an error.
    #[serde(default = "default_connection_timeout")]
    pub connection_timeout: u64,
    /// Time to wait for a query response before canceling the query and
    /// returning an error.
    #[serde(default = "default_statement_timeout")]
    pub statement_timeout: u64,
    /// Number of threads to use for asynchronous operations such as connection
    /// creation.
    #[serde(default = "default_helper_threads")]
    pub helper_threads: usize,
    /// Whether to enforce that all the database connections are encrypted with TLS.
    #[serde(default = "false_value")]
    pub enforce_tls: bool,
}