use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default = "default_discord_config")]
    #[serde(alias = "bot")]
    pub discord: DiscordConfig,
    #[serde(default = "default_tracing_config")]
    pub tracing: TracingConfig,
    #[serde(default = "default_postgres_config")]
    #[serde(alias = "database")]
    #[serde(alias = "postgresql")]
    pub postgres: PostgresConfig,
    #[serde(default = "default_voice_config")]
    #[serde(alias = "music")]
    #[serde(alias = "voice")]
    pub lavalink: VoiceConfig,
}
#[derive(Debug, Deserialize)]
pub struct DiscordConfig {
    #[serde(default = "discord_token")]
    pub token: String,
}

#[derive(Debug, Deserialize)]
pub struct TracingConfig {
    // #[serde(default = true)]
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(default = "tracing_level")]
    #[serde(alias = "level")]
    pub tracing_level: String,
}

#[derive(Debug, Deserialize)]
pub struct PostgresConfig {
    #[serde(default = "database_host")]
    pub host: String,
    #[serde(default = "database_port")]
    pub port: u16,
    #[serde(default = "database_user")]
    pub user: String,
    #[serde(default = "database_password")]
    pub password: String,
    #[serde(alias = "db", default = "database")]
    pub database: String,
}

#[derive(Debug, Deserialize)]
pub struct VoiceConfig {
    #[serde(default = "voice_host")]
    pub host: String,
    #[serde(default = "voice_port")]
    pub port: u16,
    #[serde(default = "voice_password")]
    pub password: String,
}

fn default_discord_config() -> DiscordConfig {
    DiscordConfig {
        token: discord_token(),
    }
}

fn default_voice_config() -> VoiceConfig {
    VoiceConfig {
        host: voice_host(),
        port: voice_port(),
        password: voice_password(),
    }
}

fn default_tracing_config() -> TracingConfig {
    TracingConfig {
        enabled: true,
        tracing_level: "info".to_string(),
    }
}

fn default_postgres_config() -> PostgresConfig {
    PostgresConfig {
        host: database_host(),
        port: database_port(),
        user: database_user(),
        password: database_password(),
        database: database(),
    }
}

fn default_true() -> bool {
    true
}

fn tracing_level() -> String {
    String::from("info")
}

fn discord_token() -> String {
    let token = env::var("DISCORD_TOKEN");
    if let Err(why) = token {
        eprintln!(
            "Kuso! Failed to get discord token env var `DISCORD_TOKEN` >_<: {}",
            why
        );
        return "".to_string();
    }

    token.unwrap()
}

fn database_host() -> String {
    String::from("db")
}

fn database_port() -> u16 {
    5432
}

fn database_user() -> String {
    String::from("postgres")
}

fn database_password() -> String {
    let pwd = env::var("POSTGRES_PASSWORD");
    if let Err(why) = pwd {
        eprintln!(
            "Kuso! Failed to get database password env var `POSTGRES_PASSWORD` >_<: {}",
            why
        );
        return "".to_string();
    }

    pwd.unwrap()
}

fn database() -> String {
    String::from("postgres")
}

fn voice_host() -> String {
    String::from("127.0.0.1")
}

fn voice_password() -> String {
    let pwd = env::var("LAVALINK_PASSWORD");
    if let Err(why) = pwd {
        eprintln!(
            "Kuso! Failed to get lavalink password env var `LAVALINK_PASSWORD` >_<: {}",
            why
        );
        return "".to_string();
    }

    pwd.unwrap()
}

fn voice_port() -> u16 {
    8080u16
}
