use std::{env, error::Error as StdError};

use crate::components::get_commands;
use config::Config;
use dotenv::dotenv;
use error::Error;
use global_data::Data;
use songbird::SerenityInit;
use tokio::{fs::File, io::AsyncReadExt};
use tracing::Level;
use tracing_log::LogTracer;
use tracing_subscriber::FmtSubscriber;

mod components;
mod config;
mod database;
mod error;
mod event_listeners;
mod global_data;
mod utility;

pub type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn StdError + Send + Sync + 'static>> {
    dotenv().ok();
    let config_path = env::var("CONFIG_PATH").unwrap_or_else(|_| "./Config.toml".to_string());
    let mut config_file = File::open(config_path).await?;
    let mut config_content = String::new();
    config_file.read_to_string(&mut config_content).await?;
    let config = toml::from_str::<Config>(&config_content)?;

    if config.tracing.enabled {
        LogTracer::init()?;
        let level = match config.tracing.tracing_level.as_str() {
            "error" => Level::ERROR,
            "warn" => Level::WARN,
            "info" => Level::INFO,
            "debug" => Level::DEBUG,
            "trace" => Level::TRACE,
            _ => Level::INFO,
        };
        let subscriber = FmtSubscriber::builder().with_max_level(level).finish();
        tracing::subscriber::set_global_default(subscriber)?;
    }

    let options: poise::FrameworkOptions<Data, Error> = poise::FrameworkOptions {
        commands: get_commands(),
        ..Default::default()
    };

    let framework = poise::Framework::build()
        .token(&config.discord.token)
        .options(options)
        .client_settings(|c| c.register_songbird())
        .user_data_setup(move |ctx, ready, _framework| {
            Box::pin(async move { Data::new(ctx, ready, &config).await })
        });

    framework.run().await?;

    Ok(())
}
