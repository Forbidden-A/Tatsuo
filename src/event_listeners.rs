use lavalink_rs::model::{
    PlayerDestroyed, PlayerUpdate, Stats, TrackException, TrackFinish, TrackStart, TrackStuck,
    WebSocketClosed,
};
use lavalink_rs::LavalinkClient;

pub struct LavalinkHandler;

#[lavalink_rs::async_trait]
impl lavalink_rs::gateway::LavalinkEventHandler for LavalinkHandler {
    async fn stats(&self, _client: LavalinkClient, _event: Stats) {}

    async fn player_update(&self, client: LavalinkClient, event: PlayerUpdate) {
        let nodes = client.nodes().await;
        let node = nodes.iter().find(|it| it.guild.as_u64() == event.guild_id.as_u64());
        // update_music_menu(node, event)?
        println!("{:?}", event)
    }

    async fn track_start(&self, _client: LavalinkClient, _event: TrackStart) {}

    async fn track_finish(&self, _client: LavalinkClient, _event: TrackFinish) {}

    async fn track_exception(&self, client: LavalinkClient, event: TrackException) {
        let queue = client.skip(event.guild_id).await;
        if let None = queue {
            client.stop(event.guild_id).await;
        }
    }

    async fn track_stuck(&self, client: LavalinkClient, event: TrackStuck) {
        let queue = client.skip(event.guild_id).await;
        if let None = queue {
            client.stop(event.guild_id).await;
        }
    }

    async fn websocket_closed(&self, _client: LavalinkClient, event: WebSocketClosed) {
        println!("{:?}", event)
    }

    async fn player_destroyed(&self, _client: LavalinkClient, event: PlayerDestroyed) {
        println!("{:?}", event)
    }
}
