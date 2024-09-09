use crate::{Context, Result, Error, Data};
use log::info;
use poise::serenity_prelude as serenity;
use poise::serenity_prelude::Builder;
use poise::FrameworkError;

async fn create_command(
    ctx: Context<'_>,
    name: String,
    description: String,
    body: String,
) -> Result<()> {
    async fn inner(ctx_: Context<'_>) -> std::result::Result<(), FrameworkError<'_, Data, Error>> {
        info!("Executing custom command {}", ctx_.command().qualified_name);
        ctx_.say("temp body").await.unwrap();
        Ok(())
    }

    let command = poise::Command {
        name: name,
        description: Some(description),
        guild_only: true,
        slash_action: Some(|ctx_| Box::pin(async move {
            info!("Slash action");
            inner(ctx_.into()).await
        })),
        ..Default::default()
    };

    let builder = command.create_as_slash_command().unwrap();
    let guild = ctx.guild_id().unwrap();

    info!("Registing in {}", guild);
    guild.create_command(ctx, builder).await?;

    Ok(())
}

#[poise::command(slash_command, subcommands("create", "list", "info", "remove"))]
pub async fn command(
    ctx: Context<'_>,
    #[description = "Command name"] name: String,
) -> Result<()> {
    Ok(())
}

#[poise::command(slash_command)]
pub async fn create(
    ctx: Context<'_>,
    #[description = "Command name"] name: String,
    #[description = "Command description"] description: String,
    #[description = "Command body"] body: String,
) -> Result<()> {
    let response = format!("Created command: {}", name);
    create_command(ctx, name, description, body).await?;
    ctx.say(response).await?;
    Ok(())
}

#[poise::command(slash_command)]
pub async fn list(ctx: Context<'_>) -> Result<()> {
    let response = "[List]";
    ctx.say(response).await?;
    Ok(())
}

#[poise::command(slash_command)]
pub async fn info(
    ctx: Context<'_>,
    #[description = "Command name"] name: String,
) -> Result<()> {
    let response = format!("[Info] Name: {}", name);
    ctx.say(response).await?;
    Ok(())
}

#[poise::command(slash_command)]
pub async fn remove(
    ctx: Context<'_>,
    #[description = "Command name"] name: String,
) -> Result<()> {
    let response = format!("[Info] Name: {}", name);
    ctx.say(response).await?;
    Ok(())
}
