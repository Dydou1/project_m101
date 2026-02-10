use std::sync::LazyLock;

use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

use crate::load_var;

pub static URL: LazyLock<&'static str> = LazyLock::new(|| load_var!("DATABASE_URL"));

pub static POOL: LazyLock<PgPool> = LazyLock::new(|| {
    PgPoolOptions::new()
        .min_connections(4)
        .max_connections(12)
        .connect_lazy(*URL)
        .expect("db must be accessible")
});
