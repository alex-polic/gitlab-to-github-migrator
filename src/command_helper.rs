use std::{process::Command};

pub fn clone_repo(repo_name: &String) {

    let git_repo = format!("git@gitlab.com:marbleit/{0}.git", repo_name.as_str());

    let process_status = Command::new("git")
        .current_dir("./")
        .arg("clone")
        .arg("--mirror")
        .arg(git_repo)
        .status()
        .expect("Something unexpected happened");

    println!("{process_status}");
}

pub fn add_new_remote(repo_name: &String) {
    let process_status = Command::new("git")
        .current_dir(format!("./{}.git", repo_name))
        .arg("remote")
        .arg("add")
        .arg("github")
        .arg(format!("git@github.com:MarbleIT/{0}.git", repo_name))
        .status()
        .expect("Something unexpected happened");

    println!("{process_status}");
}

pub fn set_new_remote(repo_name: &String) {
    let process_status = Command::new("git")
        .current_dir(format!("./{}.git", repo_name))
        .arg("remote")
        .arg("set-url")
        .arg("--push")
        .arg("origin")
        .arg(format!("git@github.com:MarbleIT/{0}.git", repo_name))
        .status()
        .expect("Something unexpected happened");

    println!("{process_status}");
}

pub fn push_new_remote(repo_name: &String) {
    let process_status = Command::new("git")
        .current_dir(format!("./{}.git", repo_name))
        .arg("push")
        .arg("--mirror")
        .status()
        .expect("Something unexpected happened");

    println!("{process_status}");
}

pub fn delete_local_repo(repo_name: &String) {
    let process_status = Command::new("rm")
        .arg("-rf")
        .arg(format!("./{}.git", repo_name))
        .status()
        .expect("Something unexpected happened");

    println!("{process_status}");
}