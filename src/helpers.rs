const GITHUB_BASE_API: &str = "https://api.github.com/";

pub fn format_github_api_url(endpoint: &str) -> String {
    return format!("{0}/{1}", GITHUB_BASE_API, endpoint)
}