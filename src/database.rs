use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    PgPool,
};

use crate::{config::PostgresConfig, error::CommandResult};

pub async fn obtain_postgres_pool(config: &PostgresConfig) -> CommandResult<PgPool> {
    let connect_options = PgConnectOptions::new()
        .host(&config.host)
        .port(config.port)
        .username(&config.user)
        .password(&config.password)
        .database(&config.database);

    let pool = PgPoolOptions::new()
        .max_connections(20)
        .connect_with(connect_options)
        .await?;

    Ok(pool)
}
