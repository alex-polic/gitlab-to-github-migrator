use std::{process::Command};

pub fn clone_repo(repo_name: String) {

    let git_repo = format!("git@github.com:MarbleIT/{0}.git", repo_name.as_str());

    let process_status = Command::new("git")
        .current_dir("./")
        .arg("clone")
        .arg(git_repo)
        .status()
        .expect("Something unexpected happened");

    println!("{process_status}");
}