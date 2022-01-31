use crate::error::CommandResult;
use crate::Context;

pub async fn is_owner(ctx: Context<'_>) -> CommandResult<bool> {
    Ok(ctx.data().owner_ids.contains(&ctx.author().id))
}

pub async fn is_guild(ctx: Context<'_>) -> CommandResult<bool> {
    Ok(ctx.guild_id().is_some())
}
