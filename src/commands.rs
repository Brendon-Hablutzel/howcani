use anyhow::anyhow;
use futures::StreamExt;
use std::fmt::Display;
use std::io::Write;
use std::str::FromStr;
use sysinfo::System;
use tokio::pin;

use crate::creds::{self, get_creds, Creds};
use crate::models::cohere::{CohereModel, CohereModelConfig};
use crate::models::GenerativeModel;

fn get_user_input<T>(prompt: &str) -> T
where
    T: FromStr,
    <T as FromStr>::Err: Display,
{
    print!("{prompt}: ");
    std::io::stdout()
        .flush()
        .expect("should be able to flush stdout");

    let mut user_input = String::new();
    std::io::stdin()
        .read_line(&mut user_input)
        .expect("should be able to read from stdin");

    match T::from_str(user_input.trim_end()) {
        Ok(result) => result,
        Err(error) => {
            eprintln!("Error, invalid input: {error}");
            get_user_input(prompt)
        }
    }
}

pub async fn ask(query: &str) -> anyhow::Result<()> {
    let os_name =
        System::long_os_version().ok_or(anyhow!("host OS data could not be retrieved"))?;
    let preamble = format!(include_str!("preamble.txt"), os_name);

    let creds = get_creds().await?.ok_or(anyhow!(
        "no credentials file found, please add credentials using the appropriate command"
    ))?;

    let model_config = CohereModelConfig {
        api_key: creds.cohere_api_key,
    };

    let model = CohereModel::new(model_config);

    let responses_stream = model.generate_response(&preamble, query).await?;

    pin!(responses_stream);

    while let Some(text) = responses_stream.next().await {
        print!("{}", text?);
    }

    Ok(())
}

pub async fn get_censored_creds() -> anyhow::Result<()> {
    let creds = creds::get_creds()
        .await?
        .ok_or(anyhow!("no credentials found"))?;

    let cohere_api_key = creds.cohere_api_key;

    let censored_key: String = cohere_api_key
        .chars()
        .enumerate()
        .map(|(index, char)| {
            if index <= 5 {
                char
            } else {
                char::from_str("*").expect("asterisk should be valid character")
            }
        })
        .collect();

    println!("Cohere API key: {}", censored_key);

    Ok(())
}

pub async fn add_creds() -> anyhow::Result<()> {
    let cohere_api_key: String = get_user_input("Enter your Cohere API KEY");

    let new_creds = Creds { cohere_api_key };

    creds::add_creds(&new_creds).await?;

    println!("Credentials successfully written to file");

    Ok(())
}

pub async fn remove_creds() -> anyhow::Result<()> {
    creds::remove_creds().await?;

    println!("Credentials successfully removed");

    Ok(())
}
