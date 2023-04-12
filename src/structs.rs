use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RepoRequest {
    pub name: String,
    pub description: String
}

impl RepoRequest {
    pub fn new(name: String, description: String) -> RepoRequest {
       return RepoRequest { name, description };
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

impl GitlabRepo {
    pub fn new(name: String, description: Option<String>, path: String, default_branch: String) -> GitlabRepo {
        return GitlabRepo { name, description, path, default_branch };
    }
}
impl fmt::Display for GitlabRepo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})\n", self.name, self.path)
    }
}
