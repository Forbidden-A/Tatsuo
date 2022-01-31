mod music;

use crate::{error::CommandResult, utility::is_owner, Context, Data, Error};
use poise::Command;

#[poise::command(prefix_command, hide_in_help, check = "is_owner")]
async fn register(ctx: Context<'_>, #[flag] global: bool) -> CommandResult<()> {
    poise::builtins::register_application_commands(ctx, global).await?;

    Ok(())
}

pub fn get_commands() -> Vec<Command<Data, Error>> {
    vec![
        register(),
        poise::Command {
            subcommands: vec![
                music::join_command(),
                music::play_command(),
                music::search_command(),
            ],
            ..music::music_command()
        },
    ]
}
