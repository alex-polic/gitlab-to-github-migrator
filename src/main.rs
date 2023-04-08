// TO DO: Create empty repo in Github
// pull repo from gitlab
// cd to repo
// git remote set-url github git@gitlab.com:marbleit/Zoocentar.git
// git push --mirror github
// cd backwards
// remove repo

// list repos from gitlab
mod structs;
use std::str::FromStr;

use crate::structs::RepoRequest;

fn main() {

    let repo_req = RepoRequest::new(
        String::from_str("test-repo").unwrap(),
        String::from_str("Testing sserialization").unwrap()
    );
    let txt = serde_json::to_string(&repo_req).unwrap();

    println!("{txt}");
    println!("{:?}", repo_req);
}
