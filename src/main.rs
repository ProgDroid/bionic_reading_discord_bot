// use crate::commands;
use poise::serenity_prelude as serenity;
use bionic_reading_api::{bionic::{Error, Fixation, Saccade}, client::Client as BionicClient};

// type Context<'a> = poise::Context<'a, Data, Error>;
// type Error = Box<dyn std::error::Error + Send + Sync>;

const API_KEY: &str = "api_key";

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = BionicClient::new(API_KEY);

    let res = client.convert("Lorem ipsum dolor sit amet").fixation(Fixation::Weakest).saccade(Saccade::Fewest).send().await?;

    println!("{:?}", res.markdown().unwrap());

    Ok(())
}
