#![crate_type = "bin"]

mod commands;
mod config;

use crate::commands::convert::convert;
use crate::config::Config;

use anyhow::{Error, Result};
use rusty_interaction::handler::InteractionHandler;

#[actix_web::main]
async fn main() -> Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    let config = Config::load().await?;

    let mut handle = InteractionHandler::new(
        config.app_id.as_sensitive_str().parse::<u64>()?,
        config.public_key.as_sensitive_str(),
        Some(&config.token.as_sensitive_str().to_owned()),
    );

    handle.add_data(config.clone());

    handle.add_global_command("convert", convert);

    handle.run(config.port).await.map_err(Error::from)
}
