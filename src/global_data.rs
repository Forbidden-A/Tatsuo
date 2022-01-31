use std::{collections::HashMap, sync::Arc};

use crate::event_listeners::LavalinkHandler;
use crate::{config::Config, database::obtain_postgres_pool, error::CommandResult};
use poise::serenity_prelude::{Context, GuildId, MembershipState, Mutex, Ready, UserId};
use serenity::model::id::MessageId;
use sqlx::{migrate, Pool, Postgres};

#[derive(derivative::Derivative)]
#[derivative(Debug)]
pub struct Data {
    pub owner_ids: Vec<UserId>,
    #[derivative(Debug = "ignore")]
    pub lavalink: lavalink_rs::LavalinkClient,
    #[derivative(Debug = "ignore")]
    pub pg_pool: Arc<Mutex<Pool<Postgres>>>,
    pub active_music_menu_ids: Arc<Mutex<HashMap<GuildId, (MessageId, String)>>>,
}

impl Data {
    pub async fn new(ctx: &Context, ready: &Ready, config: &Config) -> CommandResult<Self> {
        let lavalink = lavalink_rs::LavalinkClient::builder(UserId(*ready.user.id.as_u64()))
            .set_password(config.lavalink.password.to_owned())
            .set_host(config.lavalink.host.to_owned())
            .set_port(config.lavalink.port.to_owned())
            .build(LavalinkHandler)
            .await?;

        let pg_pool = obtain_postgres_pool(&config.postgres).await?;
        migrate!().run(&pg_pool).await?;
        let app_info = ctx.http.get_current_application_info().await?;
        let mut owner_ids = vec![];
        if let Some(team) = app_info.team {
            owner_ids = team
                .members
                .iter()
                .filter(|m| m.membership_state == MembershipState::Accepted)
                .map(|m| m.user.id)
                .collect()
        } else {
            owner_ids.insert(0, app_info.owner.id)
        }

        Ok(Data {
            owner_ids,
            lavalink,
            pg_pool: Arc::new(Mutex::new(pg_pool)),
            active_music_menu_ids: Arc::new(Mutex::new(HashMap::new())),
        })
    }
}
