use std::{collections::HashMap};

const GITHUB_BASE_API: &str = "https://api.github.com";
const GITLAB_BASE_API: &str = "https://gitlab.com/api/v4";

pub fn format_github_api_url(endpoint: &str) -> String {
    return format!("{0}/{1}", GITHUB_BASE_API, endpoint)
}

pub fn format_gitlab_api_url(endpoint: &str) -> String {
    return format!("{0}/{1}", GITLAB_BASE_API, endpoint)
}

pub fn create_params(params_map: HashMap<&str, &str>) -> String {
    let mut params_string = "?".to_string();
    for (key, value) in &params_map {
        params_string.push_str(key);
        params_string.push_str("=");
        params_string.push_str(value);
        params_string.push_str("&");
    }

    return params_string;
}