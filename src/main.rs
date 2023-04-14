// TO DO:
// 
// 
// 
//

// implement support for subgroups
// Implement helper for repo name to repo path (Proj name to proj-name)
// Implement menu (list repos, transfer everything) p1

mod structs;
mod http_client;
mod command_helper;
mod helpers;

use command_helper::{set_new_remote, push_new_remote, delete_local_repo, clone_repo, add_new_remote};
use http_client::{create_repo, set_default_branch, list_gitlab_repos};
use structs::RepoRequest;

#[tokio::main]
async fn main() {
    migrator_template().await;
}

async fn migrator_template() {
    match list_gitlab_repos(65255486).await {
        Ok(res) => {
            for repo in res {
                println!("{}", repo);
                let request = RepoRequest::new(
                    repo.name,
                    repo.description.unwrap_or(String::from("")),
                    repo.default_branch,
                    repo.path
                );
                migrate_single_repo(&request).await;
            }
        }
        Err(err) => println!("{:?}", err.status)
    };
}

async fn migrate_single_repo(request: &RepoRequest) {

    clone_repo(&request.path);

    match create_repo(request).await {
        Ok(_) => print!("Repo created successfully"),
        Err(error) => println!("Error - code {0} - message:{1}", error.status, error.message)
    };

    add_new_remote(&request.path);
    set_new_remote(&request.path);
    push_new_remote(&request.path);

    match set_default_branch(request).await {
        Ok(_) => println!("Default branch changed successfully"),
        Err(error) => println!("Error - code {0} - message:{1}", error.status, error.message)
    };

    delete_local_repo(&request.path);
}