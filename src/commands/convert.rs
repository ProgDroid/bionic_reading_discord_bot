use std::convert::Infallible;

use crate::config::Config;
use bionic_reading_api::{
    bionic::{Fixation, Saccade},
    client::Client as BionicClient,
};
use log::error;
use rusty_interaction::{
    defer,
    handler::InteractionHandler,
    slash_command,
    types::interaction::{Context, InteractionResponse},
};

const BAD_INTERACTION_RESPONSE: &str =
    "Oops! You didn't give me everything I need to convert your text.";
const FAILED_CONVERSION_REQUEST_RESPONSE: &str = "Oops! I can't convert your text.";
const FAILED_MD_CONVERSION_RESPONSE: &str =
    "Oops! I can't seem to format the string into Markdown.";

const TEXT_INPUT_HANDLE: &str = "text_value";
const FIXATION_INPUT_HANDLE: &str = "fixation_value";
const SACCADE_INPUT_HANDLE: &str = "saccade_value";

/// Converts given text to a Bionic Reading highlighted text
#[defer]
#[slash_command]
pub async fn convert(
    handler: &InteractionHandler,
    ctx: Context,
) -> Result<InteractionResponse, Infallible> {
    let config = handler.data.get::<Config>();

    // Don't like this
    // But macro requires Infallible forcing me to always return a response
    if config.is_none() {
        error!("No config in handler data");

        let response = ctx
            .respond()
            .content(FAILED_CONVERSION_REQUEST_RESPONSE)
            .finish()?;

        return Ok(response);
    }

    let client = BionicClient::new(config.unwrap().bionic_api_key.as_sensitive_str());

    let interaction_data = ctx.interaction.data.clone();

    // Don't like this x2
    if interaction_data.is_none() {
        error!("No data in interaction");

        let response = ctx.respond().content(BAD_INTERACTION_RESPONSE).finish()?;

        return Ok(response);
    }

    let interaction_options = interaction_data.unwrap().options;

    // Don't like this x3
    if interaction_options.is_none() {
        error!("No options in interaction data");

        let response = ctx.respond().content(BAD_INTERACTION_RESPONSE).finish()?;

        return Ok(response);
    }

    let options = interaction_options.unwrap();

    let mut text: Option<String> = None;
    let mut fixation: Option<Fixation> = None;
    let mut saccade: Option<Saccade> = None;

    for option in options {
        match option.name.as_str() {
            TEXT_INPUT_HANDLE => {
                text = Some(option.value);
            }
            FIXATION_INPUT_HANDLE => {
                let fixation_value = option.value.parse::<u8>();

                if let Ok(value) = fixation_value {
                    fixation = Some(Fixation::from(value));
                }
            }
            SACCADE_INPUT_HANDLE => {
                let saccade_value = option.value.parse::<u8>();

                if let Ok(value) = saccade_value {
                    saccade = Some(Saccade::from(value));
                }
            }
            _ => continue,
        }
    }

    // Don't like this x4
    if text.is_none() {
        error!("No text to convert in interaction data");

        let response = ctx.respond().content(BAD_INTERACTION_RESPONSE).finish()?;

        return Ok(response);
    }

    let mut request = client.convert(text.unwrap());

    if let Some(fixation_value) = fixation {
        request = request.fixation(fixation_value);
    }

    if let Some(saccade_value) = saccade {
        request = request.saccade(saccade_value);
    }

    let result = request.send().await;

    let response = match result {
        Ok(content) => ctx
            .respond()
            .content(
                content
                    .markdown()
                    .unwrap_or_else(|| FAILED_MD_CONVERSION_RESPONSE.to_string()),
            )
            .finish()?,
        Err(e) => {
            error!("{e}");

            ctx.respond()
                .content(FAILED_CONVERSION_REQUEST_RESPONSE)
                .finish()?
        }
    };

    Ok(response)
}
