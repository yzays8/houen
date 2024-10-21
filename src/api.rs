use anyhow::{anyhow, bail, Result};
use reqwest::{blocking, StatusCode};
use serde_json::Value;

use crate::models::MovieData;

pub fn fetch_data(query: &str, key: &str, token: &str) -> Result<Vec<MovieData>> {
    let url = format!(
        "https://api.themoviedb.org/3/search/movie?query={query}&include_adult=false&language=en-US&api_key={key}",
    );
    let response = blocking::Client::new()
        .get(url)
        .header("Authorization", format!("Bearer {}", token))
        .send()?;
    if response.status() != StatusCode::OK {
        bail!("Failed to get response: {}", response.status());
    }
    let response: Value = response.json()?;

    let results = response
        .get("results")
        .ok_or_else(|| anyhow!("Failed to get results"))?
        .as_array()
        .ok_or_else(|| anyhow!("Failed to get results"))?;
    if results.is_empty() {
        bail!("No results found");
    }

    let mut movies = results
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

    if let Some(data) = movies.get_mut(0) {
        data.is_most_relevant = true;
    } else {
        unreachable!();
    }

    Ok(movies)
}
