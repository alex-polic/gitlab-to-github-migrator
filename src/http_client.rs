use std::{collections::HashMap, str::FromStr};

use crate::structs::RepoRequest;

pub async fn create_repo(request: RepoRequest) -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();

    let mut body_map = HashMap::new();
    body_map.insert("name", request.name);
    body_map.insert("description", request.description);
    body_map.insert("private", String::from_str("true").unwrap());

    let req = client.post("https://api.github.com/orgs/MarbleIT/repos")
        .header("Accept", "application/vnd.github+json")
        .header("User-Agent", "rust-migrator")
        .bearer_auth("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")
        .json(&body_map)
        .send()
        .await?
        .text()
        .await?;
    
    println!("{req}");

    Ok(())
}