// TO DO:
// 
// 
// 
//

// list repos from gitlab
// allow changing of primary branch
// implement support for subgroups
// Implement helper for repo name to repo path (Proj name to proj-name)

mod structs;
mod http_client;
mod command_helper;
mod helpers;
use std::{str::FromStr};

use command_helper::{set_new_remote, push_new_remote, delete_local_repo, clone_repo, add_new_remote};
use http_client::{create_repo, set_default_branch_to_develop, list_gitlab_repos};
use structs::RepoRequest;

#[tokio::main]
async fn main() {
    //migrator_template().await;
    match list_gitlab_repos(5250038).await {
        Ok(res) => {
            for repo in &res {
                println!("{}", repo)
            }
            println!("{}", res.len())
        }
        Err(err) => println!("{:?}", err.status)
    };
}

async fn migrator_template() {
    let repo_name = String::from_str("dg-client").unwrap();
    let request = RepoRequest::new(
        String::from_str("dg-client").unwrap(), 
        String::from_str("Repository for the DG client app, written in React.").unwrap()
    );

    clone_repo(&repo_name);

    match create_repo(&request).await {
        Ok(_) => print!("Repo created successfully"),
        Err(error) => println!("Error - code {0} - message:{1}", error.status, error.message)
    };

    add_new_remote(&repo_name);
    set_new_remote(&repo_name);
    push_new_remote(&repo_name);

    match set_default_branch_to_develop(&request).await {
        Ok(_) => println!("Default branch changed successfully"),
        Err(error) => println!("Error - code {0} - message:{1}", error.status, error.message)
    };

    delete_local_repo(&repo_name);

}