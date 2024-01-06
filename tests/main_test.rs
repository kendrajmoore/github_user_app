use github_user_app::{process_search_results, FRAGMENT, Repository, SearchResults};
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_BASE_URL: &str = "https://api.github.com/search/repositories";

    fn build_language_search_url(language: &str) -> String {
        format!(
            "{}?q=language:{}&sort=stars&order=desc",
            TEST_BASE_URL, utf8_percent_encode(language, FRAGMENT)
        )
    }

    fn build_stars_search_url(stars: &str) -> String {
        format!(
            "{}?q=stars:>={}&sort=stars&order=desc",
            TEST_BASE_URL, stars
        )
    }

    #[test]
    fn test_build_language_search_url() {
        let language = "rust";
        let url = build_language_search_url(language);
        assert_eq!(url, "https://api.github.com/search/repositories?q=language:rust&sort=stars&order=desc");
    }

    #[test]
    fn test_build_stars_search_url() {
        let stars = "100";
        let url = build_stars_search_url(stars);
        assert_eq!(url, "https://api.github.com/search/repositories?q=stars:>=100&sort=stars&order=desc");
    }

    #[test]
    fn test_process_search_results() {
        let repos = vec![
            Repository {
                name: "Repo1".to_string(),
                html_url: "http://example.com/repo1".to_string(),
                stargazers_count: 100,
                language: Some("Rust".to_string()),
            },
            Repository {
                name: "Repo2".to_string(),
                html_url: "http://example.com/repo2".to_string(),
                stargazers_count: 150,
                language: Some("Python".to_string()),
            },
        ];
    
        let search_results = SearchResults { items: repos };
        let formatted_results = process_search_results(search_results);
        assert_eq!(formatted_results.len(), 4);
        assert_eq!(formatted_results[0], "--------------");
        assert!(formatted_results[1].contains("Repo1"));
        assert!(formatted_results[2].contains("Repo2"));
        assert_eq!(formatted_results[3], "--------------");
    }
    

}
