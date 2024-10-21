use std::env;
use std::io::Write;

use anyhow::Result;
use arboard::Clipboard;
use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, Input, Password, Select};
use dotenv::dotenv;

use crate::api::fetch_data;
use crate::models::MovieData;

fn get_api_key() -> Result<String> {
    if dotenv().is_ok() && env::var("API_KEY").is_ok() {
        Ok(env::var("API_KEY").unwrap())
    } else {
        let k = Password::new()
            .with_prompt("Enter your API key")
            .interact()?;
        write_api_key_to_env(&k)?;
        Ok(k)
    }
}

fn get_api_token() -> Result<String> {
    if dotenv().is_ok() && env::var("API_TOKEN").is_ok() {
        Ok(env::var("API_TOKEN").unwrap())
    } else {
        let t = Password::new()
            .with_prompt("Enter your API token")
            .interact()?;
        write_api_token_to_env(&t)?;
        Ok(t)
    }
}

fn write_api_key_to_env(key: &str) -> Result<()> {
    let mut file = std::fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(".env")?;
    writeln!(file, "API_KEY={}", key)?;
    Ok(())
}

fn write_api_token_to_env(token: &str) -> Result<()> {
    let mut file = std::fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(".env")?;
    writeln!(file, "API_TOKEN={}", token)?;
    Ok(())
}

fn get_query() -> Result<String> {
    let query = Input::new().with_prompt("Enter a movie title").interact()?;
    Ok(query)
}

// Note: Depending on the size of the terminal window, the selection screen may collapse.
fn select_title(movies: &[MovieData]) -> Result<Option<String>> {
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(format!(
            "Found {} results. Select a movie title to copy to clipboard.",
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

pub fn run() -> Result<()> {
    let movies = fetch_data(&get_query()?, &get_api_key()?, &get_api_token()?)?;

    if let Some(t) = select_title(&movies)? {
        let mut clipboard = Clipboard::new()?;
        clipboard.set_text(t.as_str())?;
        println!("Copied! {}", t.green());
    } else {
        println!("No title selected, exiting...");
    }

    Ok(())
}
