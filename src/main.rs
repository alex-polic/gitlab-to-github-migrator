// TO DO: Create empty repo in Github
// pull repo from gitlab
// cd to repo
// git remote set-url github git@gitlab.com:marbleit/Zoocentar.git
// git push --mirror github
// cd backwards
// remove repo

// list repos from gitlab
mod structs;
mod http_client;
mod command_helper;
use std::{str::FromStr, process::Command};

use crate::{structs::RepoRequest};

#[tokio::main]
async fn main() {
}
