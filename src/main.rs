use std::env;
use std::error::Error;
use std::fmt;

use arboard::Clipboard;
use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use dotenv::dotenv;
use reqwest::{blocking, StatusCode};
use serde_json::Value;

struct MovieData {
    title: String,
    release_date: String,
    is_most_relevant: bool,
}

impl MovieData {
    fn new(title: String, release_date: String, is_most_relevant: bool) -> Self {
        Self {
            title,
            release_date,
            is_most_relevant,
        }
    }
}

impl fmt::Display for MovieData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_most_relevant {
            write!(
                f,
                "{} {} ({})",
                String::from("[Most relevant title]").green(),
                self.title,
                self.release_date.blue()
            )
        } else {
            write!(f, "{} ({})", self.title, self.release_date.blue())
        }
    }
}

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
    if res.get("results").unwrap().as_array().unwrap().is_empty() {
        return Err("No results found".into());
    }

    let mut options = res["results"]
        .as_array()
        .unwrap()
        .iter()
        .map(|result| {
            MovieData::new(
                result["title"].as_str().unwrap().to_owned(),
                match result["release_date"].as_str() {
                    Some("") | None => "N/A".to_owned(),
                    Some(date) => date.to_owned(),
                },
                false,
            )
        })
        .collect::<Vec<MovieData>>();

    if let Some(data) = options.get_mut(0) {
        data.is_most_relevant = true;
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

    let selected_title = options[selection.unwrap()].title.clone();
    let mut clipboard = Clipboard::new()?;
    clipboard.set_text(selected_title.as_str())?;
    println!("Selected: {}", selected_title.green());

    Ok(())
}
