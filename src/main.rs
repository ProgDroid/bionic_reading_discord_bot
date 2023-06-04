#![crate_type = "bin"]

mod commands;

use std::env;

use anyhow::{Context, Error, Result};
use poise::serenity_prelude as serenity;

const DISCORD_TOKEN_HANDLE: &str = "BIONIC_DISCORD_TOKEN";

#[tokio::main]
async fn main() -> Result<(), Error> {
    let discord_token = env::var(DISCORD_TOKEN_HANDLE)?;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![commands::convert::convert()],
            ..Default::default()
        })
        .token(discord_token)
        .intents(serenity::GatewayIntents::non_privileged())
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands)
                    .await
                    .context("Failed to register commands globally")?;

                Ok(())
            })
        });

    framework.run().await?;

    Ok(())
}
