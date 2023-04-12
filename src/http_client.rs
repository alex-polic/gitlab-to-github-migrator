use std::{collections::HashMap, str::FromStr};

use reqwest::{Error, Response};
use serde::de::DeserializeOwned;

use crate::{structs::{RepoRequest, HttpError}, helpers::format_github_api_url};

pub async fn create_repo(request: &RepoRequest) -> Result<String, HttpError> {
    let client = reqwest::Client::new();

    let mut body_map = HashMap::new();
    body_map.insert("name", request.name.clone());
    body_map.insert("description", request.description.clone());
    body_map.insert("private", String::from_str("true").unwrap());

    let url = format_github_api_url("orgs/MarbleIT/repos");

    let res = client.post(url)
        .header("Accept", "application/vnd.github+json")
        .header("User-Agent", "rust-migrator")
        .bearer_auth("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")
        .json(&body_map)
        .send()
        .await;

    let res_json = match res {
        Ok(res) => res,
        Err(_) => 
            return Err(HttpError::new(
                reqwest::StatusCode::INTERNAL_SERVER_ERROR, 
                String::from_str("Something went wrong").unwrap()
            ))
    };

    match res_json.status() {
        reqwest::StatusCode::CREATED => return Ok(res_json.text().await.unwrap()),
        error => 
            return Err(HttpError::new(
                error,
                res_json.text().await.unwrap()
            ))
    };
    

}

pub async fn set_default_branch_to_develop(request: &RepoRequest) -> Result<String, HttpError>  {
    let client = reqwest::Client::new();

    let mut body_map = HashMap::new();
    body_map.insert("name", request.name.clone());
    body_map.insert("default_branch", String::from_str("develop").unwrap());

    let url = format_github_api_url(format!("repos/MarbleIT/{0}", body_map["name"]).as_str());

    let res = client.patch(url)
        .header("Accept", "application/vnd.github+json")
        .header("User-Agent", "rust-migrator")
        .bearer_auth("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")
        .json(&body_map)
        .send()
        .await;
    
    let res_json = match res {
        Ok(res) => res,
        Err(_) => 
            return Err(HttpError::new(
                reqwest::StatusCode::INTERNAL_SERVER_ERROR, 
                String::from_str("Something went wrong").unwrap()
            ))
    };
    
    match res_json.status() {
        reqwest::StatusCode::OK => return Ok(res_json.text().await.unwrap()),
        error => 
            return Err(HttpError::new(error, res_json.text().await.unwrap()))
    };
}

// TODO: Check if it can be used
async fn _handle_respnose<T: DeserializeOwned>(res: Result<Response, Error>) -> Result<T, HttpError> {
    let res_json = match res {
        Ok(res) => res,
        Err(_) => 
            return Err(HttpError::new(
                reqwest::StatusCode::INTERNAL_SERVER_ERROR, 
                String::from_str("Something went wrong").unwrap()
            ))
    };
    
    match res_json.status() {
        reqwest::StatusCode::OK => return Ok(res_json.json::<T>().await.unwrap()),
        error => 
            return Err(HttpError::new(error, res_json.text().await.unwrap()))
    };
}