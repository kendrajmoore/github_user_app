extern crate serde_json;
use reqwest;
use serde::Deserialize;
use thiserror::Error;
use tokio::time::{sleep, Duration};
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

pub const BASE_URL: &str = "https://api.github.com/search/repositories";

pub const FRAGMENT: &AsciiSet = &CONTROLS.add(b'+').add(b'&').add(b'.').add(b'/').add(b'?');

#[derive(Error, Debug)]
pub enum MyError {
    #[error("Network request failed")]
    Network(#[from] reqwest::Error),

    #[error("Failed to parse JSON")]
    Json(#[from] serde_json::Error),
}

#[derive(Deserialize, Debug)]
pub struct Repository {
    pub name: String,
    pub html_url: String,
    pub stargazers_count: u32,
    pub language: Option<String>,
}


#[derive(Deserialize, Debug)]
pub struct SearchResults {
    pub items: Vec<Repository>,
}

pub fn build_language_search_url(language: &str) -> String {
    format!(
        "{}?q=language:{}&sort=stars&order=desc",
        BASE_URL, utf8_percent_encode(language, FRAGMENT)
    )
}

pub fn build_stars_search_url(stars: &str) -> String {
    format!(
        "{}?q=stars:>={}&sort=stars&order=desc",
        BASE_URL, stars
    )
}

pub fn build_random_repos_url() -> String {
    format!("{}/?q=stars:>=1&sort=updated&order=desc", BASE_URL)
}

pub fn process_search_results(search_results: SearchResults) -> Vec<String> {
    search_results.items.iter().enumerate().map(|(index, repo)| {
        format!("{}. {} - {} stars. URL: {}", index + 1, repo.name, repo.stargazers_count, repo.html_url)
    }).collect()
}

pub async fn fetch_repos(client: &reqwest::Client, url: &str) -> Result<SearchResults, MyError> {
    let response = client.get(url).header("User-Agent", "request").send().await?;
    let response_text = response.text().await?;

    serde_json::from_str(&response_text).map_err(MyError::Parse)
}

