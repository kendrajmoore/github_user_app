use reqwest;
use std::io;
use serde::Deserialize;
use rand::seq::SliceRandom;
use tokio::time::{sleep, Duration};
const FRAGMENT: &AsciiSet = &CONTROLS.add(b'+');
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};


#[derive(Deserialize, Debug)]
struct Repository {
    name: String,
    html_url: String,
    stargazers_count: u32,
    language: Option<String>,
}

#[derive(Deserialize, Debug)]
struct SearchResults {
    items: Vec<Repository>,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();

    loop {
        println!("GitHub Finder Menu by Numberes:");
        println!("The goal is to introduce you to new technologies");
        println!("1. Enter (1) search for repos by language");
        println!("2. Enter (2) search for top repos by stars");
        println!("3. Enter (3) to get 5 random repos");
        println!("4. Exit");

        let mut choice = String::new();
        println!("Enter your choice: ");
        io::stdin().read_line(&mut choice).expect("Failed to read line");

        match choice.trim() {
            "1" => search_by_language(&client).await?,
            "2" => search_by_stars(&client).await?,
            "3" => fetch_random_repos(&client).await?,
            "4" => {
                println!("Exiting...");
                break;
            },
            _ => println!("Invalid choice, please try again."),
        }
    }

    Ok(())
}

async fn search_by_language(client: &reqwest::Client) -> Result<(), reqwest::Error> {
    let mut language = String::new();
    println!("Enter the programming language:");
    io::stdin().read_line(&mut language).expect("Failed to read line");
    let encoded_language = utf8_percent_encode(language.trim(), FRAGMENT);

    println!("Loading....................................................");

    let url = format!(
        "https://api.github.com/search/repositories?q=language:{}&sort=stars&order=desc",
        encoded_language
    );

    let response = client.get(url)
        .header("User-Agent", "request")
        .send().await?;

    let search_results: SearchResults = response.json().await?;
    println!();
    println!("Top 3 repositories by language:");
    println!("------------------------------------------------------------");
    for (index, repo) in search_results.items.iter().take(3).enumerate() {
        println!("{}. {} - {} stars. URL: {}\n", index + 1, repo.name, repo.stargazers_count, repo.html_url);
    }
    println!("------------------------------------------------------------");
    println!();
    println!("Main menu in 10 seconds.................");
    sleep(Duration::from_secs(10)).await;
    Ok(())
}

async fn search_by_stars(client: &reqwest::Client) -> Result<(), reqwest::Error> {
    let mut stars = String::new();
    println!("Enter the minimum number of stars:");
    io::stdin().read_line(&mut stars).expect("Failed to read line");

    println!("Loading....................................................");

    let url = format!(
        "https://api.github.com/search/repositories?q=stars:>={}&sort=stars&order=desc",
        stars.trim()
    );

    let response = client.get(url)
        .header("User-Agent", "request")
        .send().await?;

    let search_results: SearchResults = response.json().await?;
    println!();
    println!("Top 5 repositories by stars:");
    println!("------------------------------------------------------------");
    for (index, repo) in search_results.items.iter().take(5).enumerate() {
        println!("{}. {} - {} stars. URL: {}\n", index + 1, repo.name, repo.stargazers_count, repo.html_url);
    }
    println!("------------------------------------------------------------");
    println!();
    println!("Main menu in 10 seconds.................");
    sleep(Duration::from_secs(10)).await;
    Ok(())
}

async fn fetch_random_repos(client: &reqwest::Client) -> Result<(), reqwest::Error> {
    let mut rng = rand::thread_rng();

    println!("Loading....................................................");

    let url = "https://api.github.com/search/repositories?q=stars:>=1&sort=updated&order=desc";
    let response = client.get(url)
        .header("User-Agent", "request")
        .send().await?;

    let mut search_results: SearchResults = response.json().await?;

    search_results.items.shuffle(&mut rng);

    println!();
    println!("5 random repositories:");
    println!("------------------------------------------------------------");
    for (index, repo) in search_results.items.iter().take(5).enumerate() {
        println!("{}. {} - {} stars. URL: {}\n", index + 1, repo.name, repo.stargazers_count, repo.html_url);
    }
    println!("------------------------------------------------------------");
    println!();
    println!("Main menu in 10 seconds.................");
    sleep(Duration::from_secs(10)).await;
    Ok(())
}
