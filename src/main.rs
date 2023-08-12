#![crate_type = "bin"]

mod commands;
mod config;
mod logger;

use crate::commands::convert::convert;
use crate::config::Config;

use anyhow::{Error, Result};
use rusty_interaction::handler::InteractionHandler;
use std::panic::set_hook;

#[actix_web::main]
async fn main() -> Result<()> {
    logger::init()?;

    set_hook(Box::new(|panic_info| {
        log::error!("{panic_info}");
    }));

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
