use std::sync::LazyLock;

use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

use crate::load_var;

/// The URL of the database, loaded from the environement
pub static URL: LazyLock<&'static str> = LazyLock::new(|| load_var!("DATABASE_URL"));

/// Pool of connections to the database
///
/// Lazily initialized and loaded
pub static POOL: LazyLock<PgPool> = LazyLock::new(|| {
    PgPoolOptions::new()
        .min_connections(1)
        .max_connections(8)
        .connect_lazy(*URL)
        .expect("db must be accessible")
});
