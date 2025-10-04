use reqwest::Client;
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
    let res: TmdbResponse = client
        .get(TMDB_API_URL)
        .header("Authorization", format!("Bearer {}", token))
        .query(&params)
        .send()
        .await?
        .json()
        .await?;
    let movies = res.results;
    if movies.is_empty() {
        return Err(Error::ResultNotFound("No results found".into()));
    }

    let mut movies = movies
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
