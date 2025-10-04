use reqwest::{Client, StatusCode};
use serde::Deserialize;

use crate::{
    error::{Error, Result},
    models::MovieData,
};

const TMDB_API_URL: &str = "https://api.themoviedb.org/3/search/movie";

#[derive(Debug, Deserialize)]
pub struct TmdbMovie {
    pub title: String,
    pub release_date: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TmdbResponse {
    pub results: Vec<TmdbMovie>,
}

pub async fn fetch_data(
    client: &Client,
    query: &str,
    key: &str,
    token: &str,
) -> Result<Vec<MovieData>> {
    let params = [
        ("query", query),
        ("include_adult", "false"),
        ("language", "en-US"),
        ("api_key", key),
    ];
    let res = client
        .get(TMDB_API_URL)
        .header("Authorization", format!("Bearer {}", token))
        .query(&params)
        .send()
        .await?;
    if res.status() != StatusCode::OK {
        // bail!("Failed to get response: {}", res.status());
        return Err(Error::Other(format!(
            "Failed to get response: {}",
            res.status()
        )));
    }
    let response: TmdbResponse = res.json().await?;
    if response.results.is_empty() {
        return Err(Error::ResultNotFound("No results found".into()));
    }

    let mut movies = response
        .results
        .iter()
        .map(|result| {
            MovieData::new(
                result.title.to_owned(),
                match result.release_date.as_deref() {
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
