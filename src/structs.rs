use std::{fmt};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RepoRequest {
    pub name: String,
    pub description: String,
    pub default_branch: String,
    pub path: String
}

impl RepoRequest {
    pub fn new(name: String, description: String, default_branch: String, path: String) -> RepoRequest {
       return RepoRequest { name, description, default_branch, path };
    }
}

pub struct HttpError {
    pub status: reqwest::StatusCode,
    pub message: String
}

impl HttpError {
    pub fn new(status: reqwest::StatusCode, message: String) -> HttpError{
        return HttpError { status, message};
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GitlabRepo {
    pub name: String,
    pub description: Option<String>,
    pub path: String,
    pub default_branch: String
}

impl fmt::Display for GitlabRepo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})\n", self.name, self.path)
    }
}
