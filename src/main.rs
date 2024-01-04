use std::io;
use github_user_app::build_language_search_url;
use github_user_app::build_random_repos_url;
use github_user_app::build_stars_search_url;
use github_user_app::fetch_repos; 
use github_user_app::process_search_results;


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
            "1" => {
                let language = get_user_input("Enter the programming language:");
                let url = build_language_search_url(&language);
                let search_results = fetch_repos(&client, &url).await?;
                let formatted_results = process_search_results(search_results);
                display_results(&formatted_results);
            },
            "2" => {
                let stars = get_user_input("Enter the minimum number of stars:");
                let url = build_stars_search_url(&stars);
                let search_results = fetch_repos(&client, &url).await?;
                let formatted_results = process_search_results(search_results);
                display_results(&formatted_results);
            },
            "3" => {
                let url = build_random_repos_url();
                let search_results = fetch_repos(&client, &url).await?;
                let formatted_results = process_search_results(search_results);
                display_results(&formatted_results);
            },
            "4" => {
                println!("Exiting...");
                break;
            },
            _ => println!("Invalid choice, please try again."),
        }
        
    }

    Ok(())
}

fn get_user_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}


fn display_results(results: &[String]) {
    for result in results {
        println!("{}", result);
    }
}



  

  


  