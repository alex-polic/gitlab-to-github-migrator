use std::{collections::{HashMap}, str::FromStr};

use reqwest::{Error, Response};
use serde::de::DeserializeOwned;

use crate::{structs::{RepoRequest, HttpError, GitlabRepo}, helpers::{format_github_api_url, format_gitlab_api_url, create_params}};

//
// Github API calls section
//

pub async fn create_repo(request: &RepoRequest) -> Result<String, HttpError> {
    let client = reqwest::Client::new();

    let mut body_map: HashMap<&str, &str> = HashMap::new();
    body_map.insert("name", &request.path);
    body_map.insert("description", &request.description);
    body_map.insert("private", "true");

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

pub async fn set_default_branch(request: &RepoRequest) -> Result<String, HttpError>  {
    let client = reqwest::Client::new();

    let mut body_map = HashMap::new();
    body_map.insert("name", request.path.clone());
    body_map.insert("default_branch", request.default_branch.clone());

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

//
// Gitlab API calls section
//

pub async fn list_gitlab_repos(group_id: i32) -> Result<Vec<GitlabRepo>, HttpError>  {
    let client = reqwest::Client::new();

    let mut url = format_gitlab_api_url(format!("groups/{0}/projects", group_id).as_str());

    let mut params = HashMap::new();
    params.insert("private_token", "glpat-5z27nRsegM4TX5UJe1zN");
    params.insert("pagination", "keyset");
    params.insert("per_page", "50");
    params.insert("order_by", "id");
    params.insert("sort", "asc");
    params.insert("page", "1");

    let params = create_params(params);
    
    url.push_str(&params);

    let res = client.get(url)
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
    let mut repos: Vec<GitlabRepo> = Vec::new();

    let pages_header = res_json.headers().get("X-Total-Pages").unwrap().to_str().unwrap();
    let pages_header = i32::from_str(pages_header).unwrap() + 1;

    match res_json.status() {
        reqwest::StatusCode::OK => {
            let mut repos_from_page = res_json.json::<Vec<GitlabRepo>>().await.unwrap();
            repos.append(&mut repos_from_page);
        },
        error => 
            return Err(HttpError::new(error, res_json.text().await.unwrap()))
    };

    for i in 2..pages_header {
        let mut url = format_gitlab_api_url(format!("groups/{0}/projects", group_id).as_str());
        
        let page_as_str = i.to_string();

        let mut params = HashMap::new();
        params.insert("private_token", "glpat-5z27nRsegM4TX5UJe1zN");
        params.insert("pagination", "keyset");
        params.insert("per_page", "50");
        params.insert("order_by", "id");
        params.insert("sort", "asc");
        params.insert("page", page_as_str.as_str());

        let params = create_params(params);
        
        url.push_str(&params);

        let res = client.get(url)
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
            reqwest::StatusCode::OK => {
                let mut repos_from_page = res_json.json::<Vec<GitlabRepo>>().await.unwrap();
                repos.append(&mut repos_from_page);
            },
            error => 
                return Err(HttpError::new(error, res_json.text().await.unwrap()))
        };
    }

    return Ok(repos);
    
}
