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