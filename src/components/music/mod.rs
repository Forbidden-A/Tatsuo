mod join;
mod play;

use std::{time::Duration, usize};

use lavalink_rs::model::Track;
use poise::{
    serenity_prelude::{
        ChannelId, ChannelType, CollectComponentInteraction, Colour, GuildChannel, Message,
    },
    ReplyHandle,
};
use rand::{prelude::StdRng, Rng, SeedableRng};
use tracing::error;

pub use join::join_command;
pub use play::{play_command, search_command};

use crate::{
    error::{CommandResult, Error},
    Context,
};

/// Base for all music related commands.
#[poise::command(slash_command, category = "Music", rename = "music")]
pub async fn music_command(_: Context<'_>) -> CommandResult<()> {
    Ok(())
}

async fn join_channel(
    context: &Context<'_>,
    channel: Option<GuildChannel>,
) -> CommandResult<ChannelId> {
    let guild = context.guild().unwrap();
    let guild_id = guild.id;

    let channel_id = match channel.as_ref().map(|c| c.kind) {
        Some(ChannelType::Voice) => Some(channel.unwrap().id),
        _ => guild
            .voice_states
            .get(&context.author().id)
            .and_then(|voice_state| voice_state.channel_id),
    };

    let connect_to = match channel_id {
        Some(channel) => channel,
        None => {
            return Err(Error::Join("You are not in a voice channel".to_string()));
        }
    };

    let manager = songbird::get(context.discord()).await.unwrap().clone();

    let (_, handler) = manager.join_gateway(guild_id, connect_to).await;

    match handler {
        Ok(connection_info) => {
            let lava_client = context.data().lavalink.clone();
            lava_client
                .create_session_with_songbird(&connection_info)
                .await?;

            Ok(connect_to)
        }
        Err(why) => {
            error!("Error joining voice channel: {}", why);
            Err(Error::Join("Error joining the channel".to_string()))
        }
    }
}

async fn play_track(
    context: &Context<'_>,
    track: Track,
    reply_handle: ReplyHandle<'_>,
) -> CommandResult<()> {
    let guild_id = context.guild_id().unwrap();

    if songbird::get(context.discord())
        .await
        .unwrap()
        .get(guild_id)
        .is_none()
    {
        join_channel(context, None).await?;
    }

    let manager = songbird::get(context.discord()).await.unwrap().clone();

    if let Some(_handle_lock) = manager.get(guild_id) {
        let lava_client = context.data().lavalink.clone();
        lava_client
            .play(guild_id, track.clone())
            .requester(context.author().id)
            .queue()
            .await?;

        let mut position = 1;

        if let Some(node) = lava_client.nodes().await.get_mut(&guild_id.0) {
            position = node.queue.len() - 1;
        };

        let track_info = &track.info.clone().unwrap();
        let mut range = StdRng::seed_from_u64(track_info.length.clone());
        let random_colour = Colour::new(range.gen_range(0x0..0xFFFFFF));

        reply_handle
            .message()
            .await?
            .edit(context.discord(), |r| {
                r.embed(|e| {
                    e.title(&track_info.title)
                        .colour(random_colour)
                        .thumbnail(format!(
                            "https://i.ytimg.com/vi/{}/default.jpg",
                            &track_info.identifier
                        ))
                        .url(&track_info.uri)
                        .footer(|f| {
                            f.text(format!("Submitted by {}", context.author().tag()))
                                .icon_url(
                                    context
                                        .author()
                                        .avatar_url()
                                        .unwrap_or_else(|| context.author().default_avatar_url()),
                                )
                        })
                        .description(format!(
                            "• **Uploader**: {uploader}\n• **Length**:   {length}",
                            uploader = &track_info.author,
                            length = {
                                let length = &track_info.length / 1000;

                                let minutes = length % 3600 / 60;
                                let seconds = length % 3600 % 60;

                                if length >= 3600 {
                                    let hours = length / 3600;
                                    format!("{}:{:02}:{:02}", hours, minutes, seconds)
                                } else {
                                    format!("{:02}:{:02}", minutes, seconds)
                                }
                            },
                        ))
                })
                .content(format!("Added to queue, with position {}", position))
                .components(|c| c)
            })
            .await?;
    } else {
        reply_handle.message().await?.edit(context.discord(), |e| e.content("Please connect the bot to the voice channel you are currently in using the `join` command.")).await?;
    }

    Ok(())
}

async fn search_track<'a>(
    context: &'a Context<'_>,
    query: String,
) -> CommandResult<Option<(Track, ReplyHandle<'a>)>> {
    if query.is_empty() {
        return Err(Error::RequiredArgument(
            "Missing the search query or URL".to_string(),
        ));
    }

    let sel_menu_id = uuid::Uuid::new_v4().to_string();

    let lava_client = context.data().lavalink.clone();

    let query_information = lava_client.auto_search_tracks(&query).await?;

    if query_information.tracks.is_empty() {
        context
            .send(|reply| {
                reply.content(format!(
                    "Could not find any results for query or url {}",
                    query
                ))
            })
            .await?;
        return Ok(None);
    }

    let m = context
        .send(|r| {
            r.content(format!("Please select a track to play."))
                .components(|c| {
                    c.create_action_row(|row| {
                        row.create_select_menu(|m| {
                            m.custom_id(sel_menu_id.clone())
                                .max_values(1)
                                .min_values(1)
                                .options(|opts| {
                                    for (i, track) in query_information.tracks.iter().enumerate() {
                                        let track_info = track.info.clone().unwrap();
                                        opts.create_option(|o| {
                                            o.label(track_info.title)
                                                .description(format!(
                                                    "{}. {}",
                                                    i, track_info.author
                                                ))
                                                .value(i)
                                        });
                                        if i == 24 {
                                            break;
                                        }
                                    }
                                    opts
                                })
                        })
                    })
                })
        })
        .await?;

    let mci = CollectComponentInteraction::new(context.discord())
        .author_id(context.author().id)
        .channel_id(context.channel_id())
        .timeout(Duration::from_secs(300))
        .filter(move |mci| mci.data.custom_id == sel_menu_id)
        .await;

    if let Some(m) = m {
        if let Some(mci) = mci {
            mci.defer(context.discord()).await?;
            let track = &query_information.tracks[mci.data.values[0].parse::<usize>().unwrap()];
            return Ok(Some((track.clone(), m)));
        } else {
            let message = m.message().await?;
            message.delete(context.discord()).await.ok();
        }
    }

    Ok(None)
}
