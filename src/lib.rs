extern crate serde_json;
use reqwest;
use serde::Deserialize;
use thiserror::Error;
use reqwest::Error as ReqwestError;
use serde_json::Error as SerdeJsonError;
use tokio::time::{sleep, Duration};
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

pub const FRAGMENT: &AsciiSet = &CONTROLS.add(b'+').add(b'&').add(b'.').add(b'/').add(b'?');

pub const BASE_URL: &str = "https://api.github.com/search/repositories";


#[derive(Error, Debug)]
pub enum MyError {
    #[error("Network error: {0}")]
    Network(#[from] ReqwestError),

    #[error("Parsing error: {0}")]
    Parse(#[from] SerdeJsonError),
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

pub fn build_license_filter() -> String {
    "license:mit".to_string()
}


pub fn build_language_search_url(language: &str) -> String {
    let license_query = build_license_filter();
    format!(
        "{}?q=language:{}+{}&sort=stars&order=desc",
        BASE_URL, utf8_percent_encode(language, FRAGMENT), license_query
    )
}

pub fn build_stars_search_url(stars: &str) -> String {
    let license_query = build_license_filter();
    format!(
        "{}?q=stars:>={}+{}&sort=stars&order=desc",
        BASE_URL,
        stars,
        license_query
    )
}

pub fn build_random_repos_url() -> String {
    let license_query = build_license_filter();
    format!(
        "{}?q=stars:>=1+{}&sort=updated&order=desc",
        BASE_URL,
        license_query
    )
}

pub fn process_search_results(search_results: SearchResults) -> Vec<String> {
    let mut results = Vec::new();
    results.push("--------------".to_string());
    for (index, repo) in search_results.items.iter().enumerate() {
        results.push(format!(
            "{}. {} - {} stars. URL: {}",
            index + 1, repo.name, repo.stargazers_count, repo.html_url
        ));
    }
    results.push("--------------".to_string());

    results
}

pub async fn fetch_repos(client: &reqwest::Client, url: &str) -> Result<SearchResults, MyError> {
    let response = client.get(url).header("User-Agent", "request").send().await?;
    let response_text = response.text().await?;
    serde_json::from_str(&response_text).map_err(MyError::Parse)
}

