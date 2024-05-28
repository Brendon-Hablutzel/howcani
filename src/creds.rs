use dirs::config_dir;
use serde::{Deserialize, Serialize};
use std::{error::Error, path::PathBuf};

const CREDS_FILE_NAME: &'static str = "howto-cli-creds.toml";

#[derive(Deserialize, Serialize)]
pub struct Creds {
    pub cohere_api_key: String,
}

fn get_creds_file_location() -> Result<PathBuf, Box<dyn Error>> {
    let config_dir = config_dir().ok_or("user config directory not found")?;
    Ok(config_dir.join(CREDS_FILE_NAME))
}

pub async fn get_creds() -> Result<Option<Creds>, Box<dyn Error>> {
    let creds_file = get_creds_file_location()?;

    if !creds_file.try_exists()? {
        return Ok(None);
    };

    let creds_str = tokio::fs::read_to_string(&creds_file).await?;

    let creds = toml::from_str(&creds_str)?;

    Ok(Some(creds))
}

pub async fn add_creds(creds: &Creds) -> Result<(), Box<dyn Error>> {
    let creds_file = get_creds_file_location()?;

    let creds_str = toml::to_string_pretty(creds)?;

    tokio::fs::write(&creds_file, &creds_str).await?;

    Ok(())
}

pub async fn remove_creds() -> Result<(), Box<dyn Error>> {
    let creds_file = get_creds_file_location()?;

    if !creds_file.try_exists()? {
        return Err("no credentials found".into());
    }

    tokio::fs::remove_file(&creds_file).await?;

    Ok(())
}
