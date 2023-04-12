use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RepoRequest {
    pub name: String,
    pub description: String
}

impl RepoRequest {
    pub fn new(name: String, description: String) -> RepoRequest {
       return RepoRequest { name: name, description: description };
    }
}

pub struct HttpError {
    pub status: reqwest::StatusCode,
    pub message: String
}

impl HttpError {
    pub fn new(status: reqwest::StatusCode, message: String) -> HttpError{
        return HttpError { status: status, message: message};
    }
}