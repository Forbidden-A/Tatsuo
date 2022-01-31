use crate::{
    components::music::{play_track, search_track},
    error::{CommandResult, Error},
    utility::is_guild,
    Context,
};

/// Add a song to the queue.
///
/// Usage: With query: `/play I Love You So`
/// or, with url: `/play https://www.youtube.com/watch?v=NwFVSclD_uc`
#[poise::command(category = "Music", slash_command, check = "is_guild", rename = "play")]
pub async fn play_command(
    context: Context<'_>,
    #[description = "URL to play or query to search"] query: String,
) -> CommandResult {
    if query.is_empty() {
        return Err(Error::RequiredArgument(
            "Missing the search query or URL".to_string(),
        ));
    }

    let lava_client = context.data().lavalink.clone();

    let m = context.say(format!("Searching for {}", query)).await?;
    if let Some(m) = m {
        let query_information = lava_client.auto_search_tracks(&query).await?;

        if query_information.tracks.is_empty() {
            context
                .send(|reply| {
                    reply.ephemeral(true).content(format!(
                        "Could not find any results for query or url {}",
                        query
                    ))
                })
                .await?;
            return Ok(());
        } else if query.starts_with("http") && query_information.tracks.len() > 1 {
            context
                .send(|m| {
                    m.content(
                        "If you would like to play the entire playlist, use `playlist` instead.",
                    );
                    m.ephemeral(true);
                    m
                })
                .await?;
        }
        play_track(&context, query_information.tracks[0].clone(), m).await?
    }
    Ok(())
}

/// Add a song to the queue.
///
/// Usage: `/search I Love You So`
#[poise::command(
    category = "Music",
    slash_command,
    check = "is_guild",
    rename = "search"
)]
pub async fn search_command(
    context: Context<'_>,
    #[description = "Query to search"] query: String,
) -> CommandResult {
    let track = search_track(&context, query).await;

    if let Err(e) = track {
        return Err(e);
    }

    if let Some((track, reply_handle)) = track.unwrap() {
        return play_track(&context, track, reply_handle).await;
    }

    Ok(())
}
