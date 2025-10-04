use std::{env, fs, io::Write};

use arboard::Clipboard;
use colored::Colorize;
use dialoguer::{Input, Password, Select, theme::ColorfulTheme};
use dotenv::dotenv;
use reqwest::Client;

use crate::{api::fetch_data, error::Result, models::MovieData};

const ENV_PATH: &str = ".env";

#[derive(Debug)]
struct Config {
    api_key: String,
    api_token: String,
}

impl Config {
    pub fn load() -> Result<Self> {
        if dotenv().is_err() {
            println!("No .env file found, creating one...");
            fs::File::create(ENV_PATH)?;
        }

        let api_key = match env::var("API_KEY") {
            Ok(v) => v,
            Err(_) => {
                let v = Self::get_env_entry_with_prompt("Enter your API key")?;
                Self::write_env_entry("API_KEY", &v)?;
                v
            }
        };
        let api_token = match env::var("API_TOKEN") {
            Ok(v) => v,
            Err(_) => {
                let v = Self::get_env_entry_with_prompt("Enter your API token")?;
                Self::write_env_entry("API_TOKEN", &v)?;
                v
            }
        };

        Ok(Self { api_key, api_token })
    }

    fn get_env_entry_with_prompt(prompt: &str) -> Result<String> {
        Password::new()
            .with_prompt(prompt)
            .interact()
            .map_err(Into::into)
    }

    fn write_env_entry(key: &str, value: &str) -> Result<()> {
        if let Ok(env_file) = fs::read_to_string(ENV_PATH)
            && env_file
                .lines()
                .any(|line| line.split('=').next() == Some(key))
        {
            return Ok(());
        }

        let mut file = fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(ENV_PATH)?;
        writeln!(file, "{}={}", key, value)?;

        Ok(())
    }
}

fn get_query() -> Result<String> {
    Input::new()
        .with_prompt("Enter a film title")
        .interact()
        .map_err(Into::into)
}

// Note: Depending on the size of the terminal window, the selection screen may collapse.
fn select_title(movies: &[MovieData]) -> Result<Option<String>> {
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(format!(
            "Found {} results. Select a film title to copy to clipboard.",
            movies.len()
        ))
        .report(false)
        .items(movies)
        .default(0)
        .interact_opt()?;

    if let Some(index) = selection {
        Ok(Some(movies[index].title.clone()))
    } else {
        Ok(None)
    }
}

pub async fn run() -> Result<()> {
    let config = Config::load()?;
    let movies = fetch_data(
        &Client::new(),
        &get_query()?,
        &config.api_key,
        &config.api_token,
    )
    .await?;

    if let Some(t) = select_title(&movies)? {
        let mut clipboard = Clipboard::new()?;
        clipboard.set_text(t.as_str())?;
        println!("Copied! {}", t.green());
    } else {
        println!("No title selected, exiting...");
    }

    Ok(())
}
