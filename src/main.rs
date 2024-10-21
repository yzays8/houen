use std::env;
use std::error::Error;

use arboard::Clipboard;
use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use dotenv::dotenv;
use reqwest::{blocking, StatusCode};
use serde_json::Value;

fn main() -> Result<(), Box<dyn Error>> {
    dotenv()?;
    let token = env::var("API_TOKEN")?;
    let key = env::var("API_KEY")?;

    let query: String = Input::new().with_prompt("Enter a movie title").interact()?;
    let res = blocking::Client::new()
        .get(format!(
            "https://api.themoviedb.org/3/search/movie?query={query}&include_adult=false&language=en-US&api_key={key}",
        ))
        .header("Authorization", format!("Bearer {}", token))
        .send()?;

    if res.status() != StatusCode::OK {
        return Err(format!("Failed to get response: {}", res.status()).into());
    }

    let res: Value = res.json()?;
    if res.get("results").is_none() {
        return Err("No results found".into());
    }

    let mut options = res["results"]
        .as_array()
        .unwrap()
        .iter()
        .map(|result| result["title"].as_str().unwrap().to_owned())
        .collect::<Vec<String>>();

    let first_title = options[0].clone();
    if let Some(title) = options.get_mut(0) {
        *title = format!(
            "{} {}",
            String::from("[Most relevant title]").green(),
            title
        )
    } else {
        unreachable!();
    }

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(format!(
            "Found {} results. Select a movie title to copy to clipboard.",
            options.len()
        ))
        .report(false)
        .items(&options)
        .default(0)
        .interact_opt()?;

    if selection.is_none() {
        println!("No title selected, exiting...");
        return Ok(());
    }

    let selected_item = if selection.unwrap() == 0 {
        first_title
    } else {
        options[selection.unwrap()].clone()
    };
    let mut clipboard = Clipboard::new()?;
    clipboard.set_text(selected_item.as_str())?;
    println!("Selected: {}", selected_item.blue());

    Ok(())
}
