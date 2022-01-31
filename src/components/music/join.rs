use poise::serenity_prelude::GuildChannel;

use crate::{components::music::join_channel, error::CommandResult, Context};

/// Join the voice channel you are currently in or the provided argument.
#[poise::command(
    slash_command,
    category = "Music",
    check = "crate::utility::is_guild",
    rename = "join",
    ephemeral
)]
pub async fn join_command(
    context: Context<'_>,
    #[description = "Voice channel to join, defaults to the channel you're in."]
    voice_channel: Option<GuildChannel>,
) -> CommandResult {
    let channel_id = join_channel(&context, voice_channel).await?;
    context.say(format!("Joined <#{}>", channel_id.0)).await?;

    Ok(())
}
