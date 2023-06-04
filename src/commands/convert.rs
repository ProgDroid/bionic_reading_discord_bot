use std::env;

use anyhow::{Context as ErrorContext, Error, Result};
use bionic_reading_api::{
    bionic::{Fixation, Saccade},
    client::Client as BionicClient,
};

type Context<'a> = poise::Context<'a, (), Error>;

const BIONIC_API_KEY_VAR_HANDLE: &str = "BIONIC_RAPID_API_KEY";
const FAILED_MD_CONVERSION_RESPONSE: &str =
    "Oops! I can't seem to format the string into Markdown.";

#[derive(poise::ChoiceParameter)]
pub enum FixationArg {
    Weakest,
    Weak,
    Average,
    Strong,
    Strongest,
}

impl From<Fixation> for FixationArg {
    fn from(value: Fixation) -> Self {
        match value {
            Fixation::Weakest => Self::Weakest,
            Fixation::Weak => Self::Weak,
            Fixation::Average => Self::Average,
            Fixation::Strong => Self::Strong,
            Fixation::Strongest => Self::Strongest,
        }
    }
}

impl From<FixationArg> for Fixation {
    fn from(value: FixationArg) -> Self {
        match value {
            FixationArg::Weakest => Self::Weakest,
            FixationArg::Weak => Self::Weak,
            FixationArg::Average => Self::Average,
            FixationArg::Strong => Self::Strong,
            FixationArg::Strongest => Self::Strongest,
        }
    }
}

impl Default for FixationArg {
    fn default() -> Self {
        Fixation::default().into()
    }
}

#[derive(poise::ChoiceParameter)]
pub enum SaccadeArg {
    Fewest,
    Few,
    Average,
    More,
    Most,
}

impl From<Saccade> for SaccadeArg {
    fn from(value: Saccade) -> Self {
        match value {
            Saccade::Fewest => Self::Fewest,
            Saccade::Few => Self::Few,
            Saccade::Average => Self::Average,
            Saccade::More => Self::More,
            Saccade::Most => Self::Most,
        }
    }
}

impl From<SaccadeArg> for Saccade {
    fn from(value: SaccadeArg) -> Self {
        match value {
            SaccadeArg::Fewest => Self::Fewest,
            SaccadeArg::Few => Self::Few,
            SaccadeArg::Average => Self::Average,
            SaccadeArg::More => Self::More,
            SaccadeArg::Most => Self::Most,
        }
    }
}

impl Default for SaccadeArg {
    fn default() -> Self {
        Saccade::default().into()
    }
}

/// Converts given text to a Bionic Reading highlighted text
#[poise::command(slash_command, track_edits)]
pub async fn convert(
    ctx: Context<'_>,
    #[description = "Enter text to convert"] input: String,
    #[description = "Fixation level (length of highlights)"] fixation: Option<FixationArg>,
    #[description = "Saccade level (amount of highlights)"] saccade: Option<SaccadeArg>,
) -> Result<(), Error> {
    let api_key = env::var(BIONIC_API_KEY_VAR_HANDLE).context("No Bionic API Key set")?;

    let client = BionicClient::new(api_key);

    let result = client
        .convert(input)
        .fixation(fixation.unwrap_or_default().into())
        .saccade(saccade.unwrap_or_default().into())
        .send()
        .await
        .context("Request to API failed")?;

    ctx.say(
        result
            .markdown()
            .unwrap_or_else(|| FAILED_MD_CONVERSION_RESPONSE.to_string()),
    )
    .await
    .context("Failed to response to convert slash command")?;

    Ok(())
}
